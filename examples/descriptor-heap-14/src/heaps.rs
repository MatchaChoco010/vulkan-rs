use anyhow::{Context, Result};
use vulkan::{Device, vk};
use vulkan_alloc::descriptor_heap::{
    DescriptorHeapLayout, DescriptorSlots, ResourceHeapPlan, SamplerHeapPlan,
};
use vulkan_example_common::device::{Gpu, Owned};
use vulkan_example_common::image::subresource_range;
use vulkan_example_common::memory::{
    Buffer, Image, MappedBuffer, buffer_device_address, create_mapped_buffer,
};

use crate::texture::TEXTURE_FORMAT;

/// A Slang `DescriptorHandle<T>` as the shaders read it from memory: the index of the
/// descriptor in its heap, and a word that stays zero.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct DescriptorHandle {
    index: u32,
    reserved: u32,
}

impl DescriptorHandle {
    fn of(slots: &DescriptorSlots, slot: u32) -> Result<Self> {
        let index = slots
            .index(slot)
            .with_context(|| format!("the heap has no slot {slot}"))?;
        Ok(Self { index, reserved: 0 })
    }
}

/// Where the descriptors go in the resource heap.
///
/// A shader turns the index in a handle into a byte offset in the heap by multiplying
/// it with the descriptor size of the kind it dereferences: the buffer descriptor size
/// for a buffer, the image descriptor size for an image. `ResourceHeapPlan` numbers its
/// slots the same way, so the index of a slot is what goes into the handle.
pub struct ResourceHeapLayout {
    pub heap: DescriptorHeapLayout,
    materials: DescriptorSlots,
    textures: DescriptorSlots,
}

impl ResourceHeapLayout {
    /// The material buffer, then `texture_count` textures, then the range the
    /// implementation reserves for itself.
    pub fn new(
        properties: &vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'_>,
        texture_count: u32,
    ) -> Result<Self> {
        let mut plan = ResourceHeapPlan::new(properties);
        let materials = plan.buffers(1)?;
        let textures = plan.images(texture_count)?;
        Ok(Self {
            heap: plan.finish()?,
            materials,
            textures,
        })
    }

    pub fn materials(&self) -> Result<DescriptorHandle> {
        DescriptorHandle::of(&self.materials, 0)
    }

    pub fn texture(&self, slot: u32) -> Result<DescriptorHandle> {
        DescriptorHandle::of(&self.textures, slot)
    }
}

/// Where the descriptor of the one sampler goes in the sampler heap.
pub struct SamplerHeapLayout {
    pub heap: DescriptorHeapLayout,
    samplers: DescriptorSlots,
}

impl SamplerHeapLayout {
    pub fn new(properties: &vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'_>) -> Result<Self> {
        let mut plan = SamplerHeapPlan::new(properties);
        let samplers = plan.samplers(1)?;
        Ok(Self {
            heap: plan.finish()?,
            samplers,
        })
    }

    pub fn sampler(&self) -> Result<DescriptorHandle> {
        DescriptorHandle::of(&self.samplers, 0)
    }
}

/// A heap is a buffer that the host writes descriptors into, here mapped for as long
/// as it lives.
pub unsafe fn create_heap(
    gpu: &Gpu,
    name: &str,
    layout: &DescriptorHeapLayout,
) -> Result<Owned<MappedBuffer>> {
    unsafe {
        create_mapped_buffer(
            gpu,
            name,
            layout.size,
            vk::BufferUsageFlags::DESCRIPTOR_HEAP_EXT | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            layout.alignment,
        )
    }
}

/// Writes the material buffer and the textures into their slots of the resource heap.
pub unsafe fn write_resource_descriptors(
    gpu: &Gpu,
    heap: &mut MappedBuffer,
    layout: &ResourceHeapLayout,
    materials: &Buffer,
    textures: &[Owned<Image>],
) -> Result<()> {
    unsafe {
        let material_range = vk::DeviceAddressRangeKHR::default()
            .address(buffer_device_address(gpu, materials.buffer))
            .size(materials.size);
        let buffer_descriptors = [vk::ResourceDescriptorInfoEXT::default()
            .ty(vk::DescriptorType::STORAGE_BUFFER)
            .data(vk::ResourceDescriptorDataEXT::address_range(
                &material_range,
            ))];

        // An image descriptor is written from what would create a view of the image;
        // no VkImageView is needed.
        let views = textures
            .iter()
            .map(|texture| {
                vk::ImageViewCreateInfo::default()
                    .image(texture.image)
                    .view_type(vk::ImageViewType::_2D)
                    .format(TEXTURE_FORMAT)
                    .subresource_range(subresource_range(vk::ImageAspectFlags::COLOR))
            })
            .collect::<Vec<_>>();
        let images = views
            .iter()
            .map(|view| {
                vk::ImageDescriptorInfoEXT::default()
                    .view(view)
                    .layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            })
            .collect::<Vec<_>>();
        let image_descriptors = images
            .iter()
            .map(|image| {
                vk::ResourceDescriptorInfoEXT::default()
                    .ty(vk::DescriptorType::SAMPLED_IMAGE)
                    .data(vk::ResourceDescriptorDataEXT::image(image))
            })
            .collect::<Vec<_>>();

        // One call per run of slots: each descriptor goes to its own slice of the heap.
        for (slots, descriptors) in [
            (&layout.materials, &buffer_descriptors[..]),
            (&layout.textures, &image_descriptors[..]),
        ] {
            let destinations = slots
                .slices(heap.bytes())
                .context("the resource heap is smaller than its layout")?
                .map(|bytes| vk::HostAddressRangeEXT::default().address(bytes))
                .collect::<Vec<_>>();
            gpu.device
                .write_resource_descriptors_ext(descriptors, &destinations)?;
        }
        Ok(())
    }
}

/// Writes the sampler that the fragment shader samples the textures with. A sampler
/// in a heap is written from its create info; there is no VkSampler.
pub unsafe fn write_sampler_descriptor(
    device: &Device,
    heap: &mut MappedBuffer,
    layout: &SamplerHeapLayout,
) -> Result<()> {
    unsafe {
        let sampler = vk::SamplerCreateInfo::default()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::REPEAT)
            .address_mode_v(vk::SamplerAddressMode::REPEAT)
            .address_mode_w(vk::SamplerAddressMode::REPEAT)
            .min_lod(0.0)
            .max_lod(0.0);
        let destination = layout
            .samplers
            .bytes(heap.bytes(), 0)
            .context("the sampler heap is smaller than its layout")?;
        device.write_sampler_descriptors_ext(
            &[sampler],
            &[vk::HostAddressRangeEXT::default().address(destination)],
        )?;
        Ok(())
    }
}

/// Makes the descriptors that the host wrote visible to the fragment shader, the only
/// stage that reads the heaps.
pub unsafe fn make_host_writes_visible(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    resource_heap: &MappedBuffer,
    sampler_heap: &MappedBuffer,
) {
    let host_write = |heap: &MappedBuffer, access| {
        vk::BufferMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::HOST)
            .src_access_mask(vk::AccessFlags2::HOST_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::FRAGMENT_SHADER)
            .dst_access_mask(access)
            .buffer(heap.buffer)
            .size(heap.size)
    };
    let barriers = [
        host_write(resource_heap, vk::AccessFlags2::RESOURCE_HEAP_READ_EXT),
        host_write(sampler_heap, vk::AccessFlags2::SAMPLER_HEAP_READ_EXT),
    ];
    unsafe {
        device.cmd_pipeline_barrier2(
            command_buffer,
            &vk::DependencyInfo::default().buffer_memory_barriers(&barriers),
        );
    }
}
