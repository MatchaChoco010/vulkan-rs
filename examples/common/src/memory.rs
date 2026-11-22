use anyhow::{Context, Result, bail};
use std::ptr::NonNull;
use vulkan::vk;
use vulkan_alloc::{Allocation, AllocationCreateDesc, AllocationScheme, MemoryLocation};

use crate::commands::submit_and_wait;
use crate::device::{Destroy, Gpu, GpuOwner, Owned};

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
            free(gpu, self.allocation);
        }
    }
}

/// Frees `allocation`. Destruction cannot return an error, so a failure is printed.
unsafe fn free(gpu: &GpuOwner, allocation: Allocation) {
    if let Err(error) = unsafe { gpu.allocator().free(allocation) } {
        eprintln!("freeing memory: {}", error.error);
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
        let allocation = gpu.allocator().allocate(AllocationCreateDesc {
            name: Some(name),
            requirements,
            location,
            linear: true,
            memory_allocate_flags,
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
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

/// Copies `bytes` to the start of a host-visible buffer.
pub unsafe fn write_buffer(gpu: &Gpu, buffer: &Buffer, bytes: &[u8]) -> Result<()> {
    unsafe {
        let mut allocator = gpu.allocator();
        let coherent = allocator
            .allocation_memory_flags(&buffer.allocation)?
            .contains(vk::MemoryPropertyFlags::HOST_COHERENT);
        let mut mapped = allocator.map(&buffer.allocation)?;
        mapped
            .byte_range_mut(0, bytes.len())
            .context("the data is larger than the buffer")?
            .copy_from_slice(bytes);
        if !coherent {
            mapped.flush()?;
        }
        Ok(())
    }
}

/// The contents of a host-visible buffer that the device has written. The writes must
/// have been made visible to the host, with a barrier to `HOST` and a wait.
pub unsafe fn read_buffer(gpu: &Gpu, buffer: &Buffer) -> Result<Vec<u8>> {
    unsafe {
        let mut allocator = gpu.allocator();
        let coherent = allocator
            .allocation_memory_flags(&buffer.allocation)?
            .contains(vk::MemoryPropertyFlags::HOST_COHERENT);
        let mut mapped = allocator.map(&buffer.allocation)?;
        if !coherent {
            mapped.invalidate()?;
        }
        Ok(mapped
            .byte_range(0, buffer.size as usize)
            .context("the buffer is larger than its allocation")?
            .to_vec())
    }
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
        let size = data.len() as u64;
        let staging = create_buffer(
            gpu,
            "staging buffer",
            size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            MemoryLocation::CpuToGpu,
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

/// A host-visible buffer that stays mapped until it is destroyed.
pub struct MappedBuffer {
    pub buffer: vk::Buffer,
    pub allocation: Allocation,
    pub size: u64,
    bytes: NonNull<u8>,
}

impl MappedBuffer {
    /// # Safety
    ///
    /// The device must not be accessing the memory.
    pub unsafe fn bytes(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.bytes.as_ptr(), self.size as usize) }
    }
}

impl Destroy for MappedBuffer {
    unsafe fn destroy(self, gpu: &GpuOwner) {
        unsafe {
            gpu.device.destroy_buffer(self.buffer, None);
            // Freeing a persistently mapped allocation also ends its mapping.
            free(gpu, self.allocation);
        }
    }
}

/// Like [`create_aligned_buffer`] in `CpuToGpu` memory, mapped for as long as the
/// buffer lives. The memory is host-coherent, so that neither side has to flush or
/// invalidate it: a buffer that is neither sparse nor protected can always be bound to
/// a host-visible and host-coherent memory type, and `CpuToGpu` takes the first such
/// type. `GpuToCpu` memory may not be coherent; read it back with [`read_buffer`]
/// instead.
pub unsafe fn create_mapped_buffer(
    gpu: &Gpu,
    name: &str,
    size: u64,
    usage: vk::BufferUsageFlags,
    alignment: u64,
) -> Result<Owned<MappedBuffer>> {
    unsafe {
        let buffer =
            create_aligned_buffer(gpu, name, size, usage, MemoryLocation::CpuToGpu, alignment)?;
        let bytes =
            map_coherent(gpu, &buffer.allocation).with_context(|| format!("mapping {name}"))?;
        let Buffer {
            buffer,
            allocation,
            size,
        } = buffer.into_inner();
        Ok(gpu.own(MappedBuffer {
            buffer,
            allocation,
            size,
            bytes,
        }))
    }
}

unsafe fn map_coherent(gpu: &Gpu, allocation: &Allocation) -> Result<NonNull<u8>> {
    unsafe {
        let mut allocator = gpu.allocator();
        if !allocator
            .allocation_memory_flags(allocation)?
            .contains(vk::MemoryPropertyFlags::HOST_COHERENT)
        {
            bail!("the memory is not host-coherent");
        }
        Ok(allocator.map_persistent(allocation)?)
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
            free(gpu, self.allocation);
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
        let allocation = gpu.allocator().allocate(AllocationCreateDesc {
            name: Some(name),
            requirements: device.get_image_memory_requirements(*image),
            location: MemoryLocation::GpuOnly,
            linear: info.tiling == vk::ImageTiling::LINEAR,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
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
