use anyhow::{Context, Result, bail};
use std::ptr::NonNull;
use vulkan::{Device, vk};
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc,
    MemoryLocation,
};

use crate::commands::submit_and_wait;
use crate::device::Gpu;

/// The allocator must be dropped before the device is destroyed.
pub unsafe fn create_allocator(gpu: &Gpu) -> Allocator {
    unsafe {
        let instance = &gpu.instance;
        Allocator::new(AllocatorCreateDesc::new(
            &gpu.device,
            instance.get_physical_device_memory_properties(gpu.physical_device),
            instance.get_physical_device_properties(gpu.physical_device),
        ))
    }
}

/// A buffer bound to memory from the allocator.
pub struct Buffer {
    pub buffer: vk::Buffer,
    pub allocation: Allocation,
    pub size: u64,
}

impl Buffer {
    pub unsafe fn destroy(self, device: &Device, allocator: &mut Allocator) -> Result<()> {
        unsafe {
            device.destroy_buffer(self.buffer, None);
            allocator.free(self.allocation)?;
        }
        Ok(())
    }
}

/// A buffer of `size` bytes in memory from `allocator`. If the buffer can be used with a
/// device address, its memory is allocated for that.
pub unsafe fn create_buffer(
    device: &Device,
    allocator: &mut Allocator,
    name: &str,
    size: u64,
    usage: vk::BufferUsageFlags,
    location: MemoryLocation,
) -> Result<Buffer> {
    unsafe { create_aligned_buffer(device, allocator, name, size, usage, location, 1) }
}

/// Like [`create_buffer`], for a buffer whose device address has to be a multiple of
/// `alignment`, as the address in a descriptor has to be.
pub unsafe fn create_aligned_buffer(
    device: &Device,
    allocator: &mut Allocator,
    name: &str,
    size: u64,
    usage: vk::BufferUsageFlags,
    location: MemoryLocation,
    alignment: u64,
) -> Result<Buffer> {
    unsafe {
        let buffer = device.create_buffer(
            &vk::BufferCreateInfo::default().size(size).usage(usage),
            None,
        )?;
        let mut requirements = device.get_buffer_memory_requirements(buffer);
        requirements.alignment = requirements.alignment.max(alignment);
        // Memory bound to a buffer whose address is taken has to be allocated for that.
        let memory_allocate_flags = if usage.contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS) {
            vk::MemoryAllocateFlags::DEVICE_ADDRESS
        } else {
            vk::MemoryAllocateFlags::empty()
        };
        let allocation = match allocator.allocate(AllocationCreateDesc {
            name: Some(name),
            requirements,
            location,
            linear: true,
            memory_allocate_flags,
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        }) {
            Ok(allocation) => allocation,
            Err(error) => {
                device.destroy_buffer(buffer, None);
                return Err(error.into());
            }
        };
        if let Err(error) =
            device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset())
        {
            let _ = allocator.free(allocation);
            device.destroy_buffer(buffer, None);
            return Err(error.into());
        }
        Ok(Buffer {
            buffer,
            allocation,
            size,
        })
    }
}

pub unsafe fn buffer_device_address(device: &Device, buffer: vk::Buffer) -> vk::DeviceAddress {
    unsafe {
        device.get_buffer_device_address(&vk::BufferDeviceAddressInfo::default().buffer(buffer))
    }
}

/// Copies `bytes` to the start of a host-visible buffer.
pub unsafe fn write_buffer(allocator: &mut Allocator, buffer: &Buffer, bytes: &[u8]) -> Result<()> {
    unsafe {
        let coherent = allocator
            .allocation_memory_flags(&buffer.allocation)?
            .contains(vk::MemoryPropertyFlags::HOST_COHERENT);
        let mut mapped = allocator.map(&buffer.allocation)?;
        mapped
            .try_byte_range_mut(0, bytes.len())
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
pub unsafe fn read_buffer(allocator: &mut Allocator, buffer: &Buffer) -> Result<Vec<u8>> {
    unsafe {
        let coherent = allocator
            .allocation_memory_flags(&buffer.allocation)?
            .contains(vk::MemoryPropertyFlags::HOST_COHERENT);
        let mut mapped = allocator.map(&buffer.allocation)?;
        if !coherent {
            mapped.invalidate()?;
        }
        Ok(mapped.byte_range(0, buffer.size as usize).to_vec())
    }
}

/// A device-local buffer that holds `data`, copied there through a staging buffer.
/// `usage` is what the buffer is used for after the upload.
pub unsafe fn upload_buffer(
    gpu: &Gpu,
    allocator: &mut Allocator,
    name: &str,
    usage: vk::BufferUsageFlags,
    data: &[u8],
) -> Result<Buffer> {
    unsafe { upload_aligned_buffer(gpu, allocator, name, usage, data, 1) }
}

/// Like [`upload_buffer`], for a buffer whose device address has to be a multiple of
/// `alignment`.
pub unsafe fn upload_aligned_buffer(
    gpu: &Gpu,
    allocator: &mut Allocator,
    name: &str,
    usage: vk::BufferUsageFlags,
    data: &[u8],
    alignment: u64,
) -> Result<Buffer> {
    unsafe {
        let device = &gpu.device;
        let size = data.len() as u64;
        let staging = create_buffer(
            device,
            allocator,
            "staging buffer",
            size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            MemoryLocation::CpuToGpu,
        )?;
        write_buffer(allocator, &staging, data)?;
        let buffer = create_aligned_buffer(
            device,
            allocator,
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
        staging.destroy(device, allocator)?;
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
    /// The buffer must be alive, and the device must not be accessing the memory.
    pub unsafe fn bytes(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.bytes.as_ptr(), self.size as usize) }
    }

    pub unsafe fn destroy(self, device: &Device, allocator: &mut Allocator) -> Result<()> {
        unsafe {
            device.destroy_buffer(self.buffer, None);
            // Freeing a persistently mapped allocation also ends its mapping.
            allocator.free(self.allocation)?;
        }
        Ok(())
    }
}

/// Like [`create_aligned_buffer`] in `CpuToGpu` memory, mapped for as long as the
/// buffer lives. The memory is host-coherent, so that neither side has to flush or
/// invalidate it: a buffer that is neither sparse nor protected can always be bound to
/// a host-visible and host-coherent memory type, and `CpuToGpu` takes the first such
/// type. `GpuToCpu` memory may not be coherent; read it back with [`read_buffer`]
/// instead.
pub unsafe fn create_mapped_buffer(
    device: &Device,
    allocator: &mut Allocator,
    name: &str,
    size: u64,
    usage: vk::BufferUsageFlags,
    alignment: u64,
) -> Result<MappedBuffer> {
    unsafe {
        let buffer = create_aligned_buffer(
            device,
            allocator,
            name,
            size,
            usage,
            MemoryLocation::CpuToGpu,
            alignment,
        )?;
        match map_coherent(allocator, &buffer.allocation) {
            Ok(bytes) => Ok(MappedBuffer {
                buffer: buffer.buffer,
                allocation: buffer.allocation,
                size: buffer.size,
                bytes,
            }),
            Err(error) => {
                let _ = buffer.destroy(device, allocator);
                Err(error.context(format!("mapping {name}")))
            }
        }
    }
}

unsafe fn map_coherent(allocator: &mut Allocator, allocation: &Allocation) -> Result<NonNull<u8>> {
    unsafe {
        if !allocator
            .allocation_memory_flags(allocation)?
            .contains(vk::MemoryPropertyFlags::HOST_COHERENT)
        {
            bail!("the memory is not host-coherent");
        }
        Ok(allocator.map_persistent(allocation)?)
    }
}

/// An image bound to device-local memory from the allocator.
pub struct Image {
    pub image: vk::Image,
    pub allocation: Allocation,
}

impl Image {
    pub unsafe fn destroy(self, device: &Device, allocator: &mut Allocator) -> Result<()> {
        unsafe {
            device.destroy_image(self.image, None);
            allocator.free(self.allocation)?;
        }
        Ok(())
    }
}

pub unsafe fn create_image(
    device: &Device,
    allocator: &mut Allocator,
    name: &str,
    info: &vk::ImageCreateInfo<'_>,
) -> Result<Image> {
    unsafe {
        let image = device.create_image(info, None)?;
        let allocation = match allocator.allocate(AllocationCreateDesc {
            name: Some(name),
            requirements: device.get_image_memory_requirements(image),
            location: MemoryLocation::GpuOnly,
            linear: info.tiling == vk::ImageTiling::LINEAR,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        }) {
            Ok(allocation) => allocation,
            Err(error) => {
                device.destroy_image(image, None);
                return Err(error.into());
            }
        };
        if let Err(error) =
            device.bind_image_memory(image, allocation.memory(), allocation.offset())
        {
            let _ = allocator.free(allocation);
            device.destroy_image(image, None);
            return Err(error.into());
        }
        Ok(Image { image, allocation })
    }
}
