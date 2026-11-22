use crate::gpu::Gpu;
use vulkan::vk;

#[test]
fn ycbcr_conversion_chains_into_sampler_and_view() {
    let Some(gpu) = Gpu::new(&[]) else { return };
    if !gpu.ycbcr {
        eprintln!("skipping: samplerYcbcrConversion is not supported");
        return;
    }
    unsafe {
        let device = &gpu.device;
        let format = vk::Format::G8_B8R8_2PLANE_420_UNORM;
        let features = gpu
            .instance
            .get_physical_device_format_properties(gpu.physical_device, format)
            .optimal_tiling_features;
        let chroma_offset = if features.contains(vk::FormatFeatureFlags::MIDPOINT_CHROMA_SAMPLES) {
            vk::ChromaLocation::MIDPOINT
        } else if features.contains(vk::FormatFeatureFlags::COSITED_CHROMA_SAMPLES) {
            vk::ChromaLocation::COSITED_EVEN
        } else {
            eprintln!("skipping: {format:?} has no Y'CbCr conversion support");
            return;
        };
        if !features.contains(vk::FormatFeatureFlags::SAMPLED_IMAGE) {
            eprintln!("skipping: {format:?} cannot be sampled");
            return;
        }

        let conversion = device
            .create_sampler_ycbcr_conversion(
                &vk::SamplerYcbcrConversionCreateInfo::default()
                    .format(format)
                    .ycbcr_model(vk::SamplerYcbcrModelConversion::YCBCR_709)
                    .ycbcr_range(vk::SamplerYcbcrRange::ITU_NARROW)
                    .x_chroma_offset(chroma_offset)
                    .y_chroma_offset(chroma_offset)
                    .chroma_filter(vk::Filter::NEAREST),
                None,
            )
            .unwrap();

        let mut sampler_conversion =
            vk::SamplerYcbcrConversionInfo::default().conversion(conversion);
        let sampler = device
            .create_sampler(
                &vk::SamplerCreateInfo::default()
                    .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                    .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                    .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                    .push_next(&mut sampler_conversion),
                None,
            )
            .unwrap();

        let image = device
            .create_image(
                &vk::ImageCreateInfo::default()
                    .image_type(vk::ImageType::_2D)
                    .format(format)
                    .extent(vk::Extent3D {
                        width: 64,
                        height: 64,
                        depth: 1,
                    })
                    .usage(vk::ImageUsageFlags::SAMPLED),
                None,
            )
            .unwrap();
        let requirements = device.get_image_memory_requirements(image);
        let memory = device
            .allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(requirements.size)
                    .memory_type_index(
                        gpu.memory_type(
                            requirements.memory_type_bits,
                            vk::MemoryPropertyFlags::empty(),
                        )
                        .unwrap(),
                    ),
                None,
            )
            .unwrap();
        device.bind_image_memory(image, memory, 0).unwrap();

        let mut view_conversion = vk::SamplerYcbcrConversionInfo::default().conversion(conversion);
        let view = device
            .create_image_view(
                &vk::ImageViewCreateInfo::default()
                    .image(image)
                    .view_type(vk::ImageViewType::_2D)
                    .format(format)
                    .subresource_range(
                        vk::ImageSubresourceRange::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .level_count(1)
                            .layer_count(1),
                    )
                    .push_next(&mut view_conversion),
                None,
            )
            .unwrap();

        device.destroy_image_view(view, None);
        device.destroy_image(image, None);
        device.free_memory(memory, None);
        device.destroy_sampler(sampler, None);
        device.destroy_sampler_ycbcr_conversion(conversion, None);
    }
}
