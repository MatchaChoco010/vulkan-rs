use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Buffer, Check, Ctx, GraphicsDesc, bytes_of, color_attachment_begin,
    color_attachment_end, image_desc, rendering_begin,
};

struct AddressBuffer {
    buffer: Buffer,
    address: vk::DeviceAddress,
}

struct Structure {
    structure: vk::AccelerationStructureKHR,
    storage: AddressBuffer,
}

fn address_buffer(
    ctx: &Ctx,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    data: Option<&[u8]>,
) -> AddressBuffer {
    unsafe {
        let info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(usage | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let buffer = ctx
            .device
            .create_buffer(&info, None)
            .check("vkCreateBuffer");
        let requirements = ctx.device.get_buffer_memory_requirements(buffer);
        let mut flags_info =
            vk::MemoryAllocateFlagsInfo::default().flags(vk::MemoryAllocateFlags::DEVICE_ADDRESS);
        let allocate = vk::MemoryAllocateInfo::default()
            .push_next(&mut flags_info)
            .allocation_size(requirements.size)
            .memory_type_index(ctx.memory_type(
                requirements.memory_type_bits,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            ));
        let memory = ctx
            .device
            .allocate_memory(&allocate, None)
            .check("vkAllocateMemory");
        ctx.device
            .bind_buffer_memory(buffer, memory, 0)
            .check("vkBindBufferMemory");
        let buffer = Buffer {
            buffer,
            memory,
            size,
        };
        if let Some(data) = data {
            ctx.buffer_write(&buffer, data);
        }
        let address_info = vk::BufferDeviceAddressInfo::default().buffer(buffer.buffer);
        let address = ctx.device.get_buffer_device_address(&address_info);
        AddressBuffer { buffer, address }
    }
}

fn structure_build(
    ctx: &Ctx,
    ty: vk::AccelerationStructureTypeKHR,
    geometry: vk::AccelerationStructureGeometryKHR<'_>,
    primitive_count: u32,
    scratch_alignment: vk::DeviceSize,
) -> Structure {
    unsafe {
        let geometries = [geometry];
        let mut build = vk::AccelerationStructureBuildGeometryInfoKHR::default()
            .ty(ty)
            .flags(vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE)
            .mode(vk::BuildAccelerationStructureModeKHR::BUILD)
            .geometries(&geometries);
        let mut sizes = vk::AccelerationStructureBuildSizesInfoKHR::default();
        ctx.device.get_acceleration_structure_build_sizes_khr(
            vk::AccelerationStructureBuildTypeKHR::DEVICE,
            &build,
            Some(&[primitive_count]),
            &mut sizes,
        );
        let storage = address_buffer(
            ctx,
            sizes.acceleration_structure_size,
            vk::BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR,
            None,
        );
        let create_info = vk::AccelerationStructureCreateInfoKHR::default()
            .buffer(storage.buffer.buffer)
            .offset(0)
            .size(sizes.acceleration_structure_size)
            .ty(ty);
        let structure = ctx
            .device
            .create_acceleration_structure_khr(&create_info, None)
            .check("vkCreateAccelerationStructureKHR");
        let scratch = address_buffer(
            ctx,
            sizes.build_scratch_size + scratch_alignment,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            None,
        );
        build = build.dst_acceleration_structure(structure).scratch_data(
            vk::DeviceOrHostAddressKHR::device_address(
                scratch.address.div_ceil(scratch_alignment) * scratch_alignment,
            ),
        );
        let ranges =
            [vk::AccelerationStructureBuildRangeInfoKHR::default()
                .primitive_count(primitive_count)];
        let command_buffer = ctx.commands_begin();
        ctx.device.cmd_build_acceleration_structures_khr(
            command_buffer,
            &[build],
            &[Some(&ranges)],
        );
        ctx.commands_submit(command_buffer);
        ctx.buffer_destroy(&scratch.buffer);
        Structure { structure, storage }
    }
}

#[allow(clippy::too_many_arguments)]
fn instance(
    scale: [f32; 2],
    translation: [f32; 3],
    custom_index: u32,
    mask: u32,
    table_offset: u32,
    flags: vk::GeometryInstanceFlagsKHR,
    reference: vk::DeviceAddress,
) -> vk::AccelerationStructureInstanceKHR {
    vk::AccelerationStructureInstanceKHR::default()
        .transform(vk::TransformMatrixKHR {
            matrix: [
                [scale[0], 0.0, 0.0, translation[0]],
                [0.0, scale[1], 0.0, translation[1]],
                [0.0, 0.0, 1.0, translation[2]],
            ],
        })
        .instance_custom_index(custom_index)
        .mask(mask)
        .instance_shader_binding_table_record_offset(table_offset)
        .flags(flags)
        .acceleration_structure_reference(reference)
}

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let vertices: [f32; 12] = [
        -0.5, -0.5, 0.0, 0.5, -0.5, 0.0, -0.5, 0.5, 0.0, 0.5, 0.5, 0.0,
    ];
    let indices: [u16; 6] = [0, 1, 2, 2, 1, 3];
    let input_usage = vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR;
    let mut structure_properties = vk::PhysicalDeviceAccelerationStructurePropertiesKHR::default();
    let mut properties =
        vk::PhysicalDeviceProperties2::default().push_next(&mut structure_properties);
    unsafe {
        ctx.instance
            .get_physical_device_properties2(ctx.physical_device, &mut properties)
    };
    let scratch_alignment = vk::DeviceSize::from(
        structure_properties.min_acceleration_structure_scratch_offset_alignment,
    );

    let vertex_buffer = address_buffer(
        ctx,
        size_of_val(&vertices) as vk::DeviceSize,
        input_usage,
        Some(bytes_of(&vertices)),
    );
    let index_buffer = address_buffer(
        ctx,
        size_of_val(&indices) as vk::DeviceSize,
        input_usage,
        Some(bytes_of(&indices)),
    );
    let triangles = vk::AccelerationStructureGeometryKHR::default()
        .geometry_type(vk::GeometryTypeKHR::TRIANGLES)
        .geometry(vk::AccelerationStructureGeometryDataKHR::triangles(
            vk::AccelerationStructureGeometryTrianglesDataKHR::default()
                .vertex_format(vk::Format::R32G32B32_SFLOAT)
                .vertex_data(vk::DeviceOrHostAddressConstKHR::device_address(
                    vertex_buffer.address,
                ))
                .vertex_stride(12)
                .max_vertex(3)
                .index_type(vk::IndexType::UINT16)
                .index_data(vk::DeviceOrHostAddressConstKHR::device_address(
                    index_buffer.address,
                )),
        ))
        .flags(vk::GeometryFlagsKHR::OPAQUE);
    let bottom = structure_build(
        ctx,
        vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL,
        triangles,
        2,
        scratch_alignment,
    );

    unsafe {
        let bottom_address_info = vk::AccelerationStructureDeviceAddressInfoKHR::default()
            .acceleration_structure(bottom.structure);
        let bottom_address = ctx
            .device
            .get_acceleration_structure_device_address_khr(&bottom_address_info);

        let instances = [
            instance(
                [0.5, 0.375],
                [-0.5, -0.5, 0.0],
                1,
                0xff,
                0,
                vk::GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE,
                bottom_address,
            ),
            instance(
                [0.75, 0.625],
                [0.375, 0.25, 0.5],
                2,
                0x01,
                5,
                vk::GeometryInstanceFlagsKHR::FORCE_OPAQUE,
                bottom_address,
            ),
            instance(
                [4.0, 4.0],
                [0.0, 0.0, -0.5],
                3,
                0x02,
                9,
                vk::GeometryInstanceFlagsKHR::empty(),
                bottom_address,
            ),
        ];
        let instance_buffer = address_buffer(
            ctx,
            size_of_val(&instances) as vk::DeviceSize,
            input_usage,
            Some(bytes_of(&instances)),
        );
        let instance_geometry = vk::AccelerationStructureGeometryKHR::default()
            .geometry_type(vk::GeometryTypeKHR::INSTANCES)
            .geometry(vk::AccelerationStructureGeometryDataKHR::instances(
                vk::AccelerationStructureGeometryInstancesDataKHR::default()
                    .array_of_pointers(false)
                    .data(vk::DeviceOrHostAddressConstKHR::device_address(
                        instance_buffer.address,
                    )),
            ))
            .flags(vk::GeometryFlagsKHR::OPAQUE);
        let top = structure_build(
            ctx,
            vk::AccelerationStructureTypeKHR::TOP_LEVEL,
            instance_geometry,
            instances.len() as u32,
            scratch_alignment,
        );

        let color = ctx.image_create(&image_desc(
            format,
            width,
            height,
            vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
            vk::ImageAspectFlags::COLOR,
        ));
        let bindings = [vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)];
        let set_layout_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        let set_layout = ctx
            .device
            .create_descriptor_set_layout(&set_layout_info, None)
            .check("vkCreateDescriptorSetLayout");
        let pool_sizes = [vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR)
            .descriptor_count(1)];
        let pool_info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(1)
            .pool_sizes(&pool_sizes);
        let pool = ctx
            .device
            .create_descriptor_pool(&pool_info, None)
            .check("vkCreateDescriptorPool");
        let set_layouts = [set_layout];
        let allocate_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(pool)
            .set_layouts(&set_layouts);
        let set = ctx
            .device
            .allocate_descriptor_sets(&allocate_info)
            .check("vkAllocateDescriptorSets")[0];
        let structures = [top.structure];
        let mut structure_write = vk::WriteDescriptorSetAccelerationStructureKHR::default()
            .acceleration_structures(&structures);
        let writes = [vk::WriteDescriptorSet::default()
            .push_next(&mut structure_write)
            .dst_set(set)
            .dst_binding(0)
            .descriptor_count(1)
            .descriptor_type(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR)];
        ctx.device.update_descriptor_sets(&writes, &[]);

        let layout = ctx.pipeline_layout(&[set_layout], &[]);
        let vertex = ctx.shader_module("ray_query.vert.spv");
        let fragment = ctx.shader_module("ray_query.frag.spv");
        let formats = [format];
        let mut rendering =
            vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
        let pipeline = ctx.graphics_pipeline(GraphicsDesc {
            blend_attachments: &[BLEND_DISABLED],
            rendering: Some(&mut rendering),
            ..GraphicsDesc::new(vertex, fragment, layout, width, height)
        });
        ctx.device.destroy_shader_module(fragment, None);
        ctx.device.destroy_shader_module(vertex, None);

        let command_buffer = ctx.commands_begin();
        color_attachment_begin(&ctx.device, command_buffer, color.image);
        rendering_begin(
            &ctx.device,
            command_buffer,
            color.view,
            [0.5, 0.5, 0.5, 1.0],
            width,
            height,
        );
        ctx.device
            .cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        ctx.device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            layout,
            0,
            &[set],
            &[],
        );
        ctx.device.cmd_draw(command_buffer, 3, 1, 0, 0);
        ctx.device.cmd_end_rendering(command_buffer);
        color_attachment_end(&ctx.device, command_buffer, color.image);
        ctx.commands_submit(command_buffer);
        ctx.readback(
            color.image,
            vk::ImageAspectFlags::COLOR,
            width,
            height,
            4,
            "color",
            "rgba8",
        );

        ctx.device.destroy_pipeline(pipeline, None);
        ctx.device.destroy_pipeline_layout(layout, None);
        ctx.device.destroy_descriptor_pool(pool, None);
        ctx.device.destroy_descriptor_set_layout(set_layout, None);
        ctx.image_destroy(&color);
        ctx.device
            .destroy_acceleration_structure_khr(top.structure, None);
        ctx.buffer_destroy(&top.storage.buffer);
        ctx.buffer_destroy(&instance_buffer.buffer);
        ctx.device
            .destroy_acceleration_structure_khr(bottom.structure, None);
    }
    ctx.buffer_destroy(&bottom.storage.buffer);
    ctx.buffer_destroy(&index_buffer.buffer);
    ctx.buffer_destroy(&vertex_buffer.buffer);
}
