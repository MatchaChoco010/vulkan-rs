use vulkan_registry::Command;

pub(super) fn wrapper(c: &Command) -> Option<String> {
    let name = c.name.as_str();
    let wrapper = match name {
        "vkBuildAccelerationStructuresKHR" => format!(
            "pub unsafe fn build_acceleration_structures_khr(&self, deferred_operation: DeferredOperationKHR, infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>], build_range_infos: &[&[AccelerationStructureBuildRangeInfoKHR]]) -> crate::vk::Result<VkResult> {{
                {pointers}
                let result = unsafe {{ self.build_acceleration_structures_khr_raw(deferred_operation, infos.len() as u32, infos.as_ptr(), pointers.as_ptr()) }};
                result.into_result()?;
                Ok(result)
            }}\n",
            pointers = pointers_to_one_array_per_info(name, "build_range_infos")
        ),
        "vkCmdBuildAccelerationStructuresKHR" => format!(
            "pub unsafe fn cmd_build_acceleration_structures_khr(&self, command_buffer: CommandBuffer, infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>], build_range_infos: &[&[AccelerationStructureBuildRangeInfoKHR]]) {{
                {pointers}
                unsafe {{ self.cmd_build_acceleration_structures_khr_raw(command_buffer, infos.len() as u32, infos.as_ptr(), pointers.as_ptr()) }}
            }}\n",
            pointers = pointers_to_one_array_per_info(name, "build_range_infos")
        ),
        "vkCmdBuildAccelerationStructuresIndirectKHR" => format!(
            "pub unsafe fn cmd_build_acceleration_structures_indirect_khr(&self, command_buffer: CommandBuffer, infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>], indirect_device_addresses: &[DeviceAddress], indirect_strides: &[u32], max_primitive_counts: &[&[u32]]) {{
                assert_eq!(infos.len(), indirect_device_addresses.len(), \"{name}: `indirect_device_addresses` must have one entry per info\");
                assert_eq!(infos.len(), indirect_strides.len(), \"{name}: `indirect_strides` must have one entry per info\");
                {pointers}
                unsafe {{ self.cmd_build_acceleration_structures_indirect_khr_raw(command_buffer, infos.len() as u32, infos.as_ptr(), indirect_device_addresses.as_ptr(), indirect_strides.as_ptr(), pointers.as_ptr()) }}
            }}\n",
            pointers = pointers_to_one_array_per_info(name, "max_primitive_counts")
        ),
        _ => return None,
    };
    Some(wrapper)
}

fn pointers_to_one_array_per_info(command: &str, arrays: &str) -> String {
    format!(
        "assert_eq!(infos.len(), {arrays}.len(), \"{command}: `{arrays}` must have one entry per info\");
        for (info, entry) in infos.iter().zip({arrays}.iter()) {{
            assert_eq!(entry.len(), info.geometry_count as usize, \"{command}: each `{arrays}` entry must have `geometry_count` elements\");
        }}
        let pointers = {arrays}.iter().map(|entry| entry.as_ptr()).collect::<Vec<_>>();"
    )
}
