use vulkan::vk;

fn main() {
    let pointers;
    {
        let geometry = vk::AccelerationStructureGeometryKHR::default();
        pointers = [&geometry];
    }
    let info = vk::AccelerationStructureBuildGeometryInfoKHR::default().geometries_ptrs(&pointers);
    let _ = info;
}
