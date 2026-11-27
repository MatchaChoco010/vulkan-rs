use anyhow::{Context, Result, anyhow, ensure};
use vulkan::vk;
use vulkan_alloc::{Allocation, AllocationCreateDesc, MemoryLocation};

use crate::commands::submit_and_wait;
use crate::device::{Destroy, Gpu, GpuOwner, Owned};
use crate::image::{DRM_FORMAT_MOD_LINEAR, drm_format_modifier, is_linear_image};

/// A buffer bound to memory from the allocator of a [`Gpu`].
pub struct Buffer {
    pub buffer: vk::Buffer,
    pub allocation: Allocation,
    pub size: u64,
}

impl Destroy for Buffer {
    unsafe fn destroy(self, gpu: &GpuOwner) {
        unsafe {
            gpu.device.destroy_buffer(self.buffer, None);
            gpu.free(self.allocation);
        }
    }
}

/// A buffer of `size` bytes in memory from the allocator. If the buffer can be used with
/// a device address, its memory is allocated for that.
pub unsafe fn create_buffer(
    gpu: &Gpu,
    name: &str,
    size: u64,
    usage: vk::BufferUsageFlags,
    location: MemoryLocation,
) -> Result<Owned<Buffer>> {
    unsafe { create_aligned_buffer(gpu, name, size, usage, location, 1) }
}

/// Like [`create_buffer`], for a buffer whose device address has to be a multiple of
/// `alignment`, as the address in a descriptor has to be.
pub unsafe fn create_aligned_buffer(
    gpu: &Gpu,
    name: &str,
    size: u64,
    usage: vk::BufferUsageFlags,
    location: MemoryLocation,
    alignment: u64,
) -> Result<Owned<Buffer>> {
    unsafe { create_buffer_with_mapping(gpu, name, size, usage, location, alignment, false) }
}

/// Like [`create_aligned_buffer`], with memory that the allocator maps when it allocates
/// it if `mapped` is set and the memory is host-visible.
unsafe fn create_buffer_with_mapping(
    gpu: &Gpu,
    name: &str,
    size: u64,
    usage: vk::BufferUsageFlags,
    location: MemoryLocation,
    alignment: u64,
    mapped: bool,
) -> Result<Owned<Buffer>> {
    unsafe {
        let device = &gpu.device;
        let buffer = gpu.own(device.create_buffer(
            &vk::BufferCreateInfo::default().size(size).usage(usage),
            None,
        )?);
        let mut requirements = device.get_buffer_memory_requirements(*buffer);
        requirements.alignment = requirements.alignment.max(alignment);
        // Memory bound to a buffer whose address is taken has to be allocated for that.
        let memory_allocate_flags = if usage.contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS) {
            vk::MemoryAllocateFlags::DEVICE_ADDRESS
        } else {
            vk::MemoryAllocateFlags::empty()
        };
        let allocation = gpu.with_allocator(|allocator| {
            allocator.allocate(AllocationCreateDesc {
                name: Some(name),
                memory_allocate_flags,
                mapped,
                ..AllocationCreateDesc::new(requirements, location, true)
            })
        })?;
        // From here the buffer and its memory go together.
        let buffer = gpu.own(Buffer {
            buffer: buffer.into_inner(),
            allocation,
            size,
        });
        device.bind_buffer_memory(
            buffer.buffer,
            buffer.allocation.memory(),
            buffer.allocation.offset(),
        )?;
        Ok(buffer)
    }
}

pub unsafe fn buffer_device_address(gpu: &Gpu, buffer: vk::Buffer) -> vk::DeviceAddress {
    unsafe {
        gpu.device
            .get_buffer_device_address(&vk::BufferDeviceAddressInfo::default().buffer(buffer))
    }
}

/// Copies `bytes` to the start of a host-visible buffer. An error when they are more
/// than the buffer holds; its allocation may be larger, but the device sees only the
/// buffer. The copy is flushed, which does nothing for host-coherent memory, such as
/// that of `CpuToGpu`, `CpuOnly` and `GpuToCpu`, and makes it visible to the device in
/// memory that is not (host-visible memory of a `GpuOnly` or `Custom` location).
pub unsafe fn write_buffer(gpu: &Gpu, buffer: &Buffer, bytes: &[u8]) -> Result<()> {
    check_fits(bytes.len(), buffer.size)?;
    gpu.with_allocator(|allocator| unsafe {
        let mut mapped = allocator.map(&buffer.allocation)?;
        mapped
            .byte_range_mut(0, bytes.len())
            .context("the buffer is larger than its allocation")?
            .copy_from_slice(bytes);
        mapped.flush()?;
        Ok(())
    })
}

/// The contents of a host-visible buffer that the device has written. The writes must
/// have been made visible to the host, with a barrier to `HOST` and a wait. The mapping
/// is invalidated before the read, which does nothing for host-coherent memory, such as
/// that of `CpuToGpu`, `CpuOnly` and `GpuToCpu`, and is needed for memory that is not
/// (host-visible memory of a `GpuOnly` or `Custom` location).
pub unsafe fn read_buffer(gpu: &Gpu, buffer: &Buffer) -> Result<Vec<u8>> {
    gpu.with_allocator(|allocator| unsafe {
        let mut mapped = allocator.map(&buffer.allocation)?;
        mapped.invalidate()?;
        Ok(mapped
            .byte_range(0, host_len(buffer.size)?)
            .context("the buffer is larger than its allocation")?
            .to_vec())
    })
}

/// Checks that `len` bytes fit into a buffer of `size` bytes.
fn check_fits(len: usize, size: u64) -> Result<()> {
    ensure!(
        u64::try_from(len).is_ok_and(|len| len <= size),
        "the data ({len} bytes) is larger than the buffer ({size} bytes)"
    );
    Ok(())
}

/// The size of a buffer for `data`. An error for no data: a buffer cannot be empty.
fn upload_size(data: &[u8]) -> Result<u64> {
    ensure!(
        !data.is_empty(),
        "there is no data to upload, and a buffer cannot be empty"
    );
    Ok(u64::try_from(data.len())?)
}

/// The device size `size` as a length on the host, whose `usize` may be narrower.
fn host_len<T: TryFrom<u64>>(size: u64) -> Result<T> {
    T::try_from(size).map_err(|_| anyhow!("{size} bytes do not fit into the host's address space"))
}

/// A device-local buffer that holds `data`, copied there through a staging buffer.
/// `usage` is what the buffer is used for after the upload.
pub unsafe fn upload_buffer(
    gpu: &Gpu,
    name: &str,
    usage: vk::BufferUsageFlags,
    data: &[u8],
) -> Result<Owned<Buffer>> {
    unsafe { upload_aligned_buffer(gpu, name, usage, data, 1) }
}

/// Like [`upload_buffer`], for a buffer whose device address has to be a multiple of
/// `alignment`.
pub unsafe fn upload_aligned_buffer(
    gpu: &Gpu,
    name: &str,
    usage: vk::BufferUsageFlags,
    data: &[u8],
    alignment: u64,
) -> Result<Owned<Buffer>> {
    unsafe {
        let device = &gpu.device;
        let size = upload_size(data)?;
        let staging = create_buffer(
            gpu,
            "staging buffer",
            size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            MemoryLocation::CpuOnly,
        )?;
        write_buffer(gpu, &staging, data)?;
        let buffer = create_aligned_buffer(
            gpu,
            name,
            size,
            usage | vk::BufferUsageFlags::TRANSFER_DST,
            MemoryLocation::GpuOnly,
            alignment,
        )?;
        submit_and_wait(gpu, |command_buffer| {
            device.cmd_copy_buffer(
                command_buffer,
                staging.buffer,
                buffer.buffer,
                &[vk::BufferCopy::default().size(size)],
            );
            // Waiting for the fence orders the copy before the work submitted later, but
            // that work also needs a memory dependency to see what the copy wrote. The
            // second scope of a barrier reaches into later submissions.
            let barrier = vk::MemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::MEMORY_READ);
            device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::DependencyFlags::empty(),
                &[barrier],
                &[],
                &[],
            );
            Ok(())
        })?;
        Ok(buffer)
    }
}

/// A host-visible buffer that stays mapped until it is destroyed: its allocation was
/// made mapped.
pub struct MappedBuffer {
    pub buffer: vk::Buffer,
    pub allocation: Allocation,
    pub size: u64,
}

impl Owned<MappedBuffer> {
    /// The mapped bytes of the buffer.
    ///
    /// # Safety
    ///
    /// The device must not be accessing the memory.
    pub unsafe fn bytes(&mut self) -> &mut [u8] {
        let buffer = self.value_mut();
        // `create_mapped_buffer` checked that the buffer fits into the mapped slice.
        let len = buffer.size as usize;
        // SAFETY: the mapping lives as long as the allocation, which `self` owns, and the
        // exclusive borrow of `self` is the only way to these bytes.
        let bytes = unsafe { buffer.allocation.mapped_slice_mut() }
            .expect("the allocation of a mapped buffer stays mapped");
        &mut bytes[..len]
    }
}

impl Destroy for MappedBuffer {
    unsafe fn destroy(self, gpu: &GpuOwner) {
        unsafe {
            gpu.device.destroy_buffer(self.buffer, None);
            // Freeing a persistently mapped allocation also ends its mapping.
            gpu.free(self.allocation);
        }
    }
}

/// Like [`create_aligned_buffer`] in `CpuToGpu` memory, mapped for as long as the
/// buffer lives. `CpuToGpu` memory is always host-coherent, so neither side has to
/// flush or invalidate it, and a buffer that is neither sparse nor protected can always
/// be bound to such memory.
pub unsafe fn create_mapped_buffer(
    gpu: &Gpu,
    name: &str,
    size: u64,
    usage: vk::BufferUsageFlags,
    alignment: u64,
) -> Result<Owned<MappedBuffer>> {
    unsafe {
        // `CpuToGpu` memory is host-visible, so the allocator maps it when it allocates it.
        let buffer = create_buffer_with_mapping(
            gpu,
            name,
            size,
            usage,
            MemoryLocation::CpuToGpu,
            alignment,
            true,
        )?;
        // The host has to be able to address all of it.
        let len: usize = host_len(size)?;
        let mapped = buffer
            .allocation
            .mapped_slice()
            .with_context(|| format!("{name} is not mapped as a whole"))?;
        ensure!(
            len <= mapped.len(),
            "the buffer is larger than its allocation"
        );
        let Buffer {
            buffer,
            allocation,
            size,
        } = buffer.into_inner();
        Ok(gpu.own(MappedBuffer {
            buffer,
            allocation,
            size,
        }))
    }
}

/// An image bound to device-local memory from the allocator of a [`Gpu`].
pub struct Image {
    pub image: vk::Image,
    pub allocation: Allocation,
}

impl Destroy for Image {
    unsafe fn destroy(self, gpu: &GpuOwner) {
        unsafe {
            gpu.device.destroy_image(self.image, None);
            gpu.free(self.allocation);
        }
    }
}

pub unsafe fn create_image(
    gpu: &Gpu,
    name: &str,
    info: &vk::ImageCreateInfo<'_>,
) -> Result<Owned<Image>> {
    unsafe {
        let device = &gpu.device;
        let image = gpu.own(device.create_image(info, None)?);
        // A DRM-modifier image is linear or not by the modifier it was given.
        let modifier = if info.tiling == vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT {
            drm_format_modifier(device, *image)?
        } else {
            DRM_FORMAT_MOD_LINEAR
        };
        let requirements = device.get_image_memory_requirements(*image);
        let allocation = gpu.with_allocator(|allocator| {
            allocator.allocate(AllocationCreateDesc {
                name: Some(name),
                ..AllocationCreateDesc::new(
                    requirements,
                    MemoryLocation::GpuOnly,
                    is_linear_image(info.tiling, modifier),
                )
            })
        })?;
        // From here the image and its memory go together.
        let image = gpu.own(Image {
            image: image.into_inner(),
            allocation,
        });
        device.bind_image_memory(
            image.image,
            image.allocation.memory(),
            image.allocation.offset(),
        )?;
        Ok(image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_upload_has_the_size_of_its_data_and_no_data_is_an_error() {
        assert!(upload_size(&[]).is_err());
        for len in [1, 2, 3, 255, 256, 4097] {
            assert_eq!(upload_size(&vec![0; len]).unwrap(), len as u64);
        }
    }

    const SIZES: [u64; 9] = [
        0,
        1,
        15,
        16,
        4096,
        u32::MAX as u64,
        u32::MAX as u64 + 1,
        (4 << 30) + 16,
        u64::MAX,
    ];

    #[test]
    fn data_fits_exactly_when_it_is_not_longer_than_the_buffer() {
        for size in SIZES {
            for len in [0, 1, 15, 16, 17, 4096, u32::MAX as usize, usize::MAX] {
                assert_eq!(
                    check_fits(len, size).is_ok(),
                    len as u128 <= size as u128,
                    "{len} bytes into {size}"
                );
            }
            if let Ok(len) = usize::try_from(size) {
                assert!(check_fits(len, size).is_ok(), "{size} bytes into {size}");
                if let Some(longer) = len.checked_add(1) {
                    assert!(
                        check_fits(longer, size).is_err(),
                        "{longer} bytes into {size}"
                    );
                }
            }
        }
    }

    #[test]
    fn a_size_becomes_a_host_length_unchanged_or_an_error() {
        for size in SIZES {
            match host_len::<u32>(size) {
                Ok(len) => assert_eq!(u64::from(len), size),
                Err(_) => assert!(size > u64::from(u32::MAX), "{size} was rejected"),
            }
            match host_len::<usize>(size) {
                Ok(len) => assert_eq!(len as u64, size),
                Err(_) => assert!(size > usize::MAX as u64, "{size} was rejected"),
            }
        }
        assert!(
            host_len::<u32>((4 << 30) + 16).is_err(),
            "4 GiB and 16 bytes became a 32-bit length"
        );
    }
}
