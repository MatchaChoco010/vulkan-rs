use crate::gpu::Gpu;
use std::ffi::CStr;
use vulkan::vk;

#[test]
fn displays_can_be_enumerated() {
    let Some(gpu) =
        Gpu::with_instance_extensions(&[vk::KHR_SURFACE_NAME, vk::KHR_DISPLAY_NAME], &[])
    else {
        return;
    };
    if !gpu.instance_extensions.contains(&vk::KHR_DISPLAY_NAME) {
        eprintln!("skipping: VK_KHR_display is not supported");
        return;
    }
    unsafe {
        let displays = gpu
            .instance
            .get_physical_device_display_properties_khr(gpu.physical_device)
            .unwrap();
        for display in &displays {
            assert!(!display.display.is_null());
            let modes = gpu
                .instance
                .get_display_mode_properties_khr(gpu.physical_device, display.display)
                .unwrap();
            let name = if display.display_name.is_null() {
                c"?"
            } else {
                CStr::from_ptr(display.display_name)
            };
            eprintln!(
                "display {name:?}: {}x{}, {} mode(s)",
                display.physical_resolution.width,
                display.physical_resolution.height,
                modes.len()
            );
        }
        let planes = gpu
            .instance
            .get_physical_device_display_plane_properties_khr(gpu.physical_device)
            .unwrap();
        eprintln!("{} display(s), {} plane(s)", displays.len(), planes.len());
    }
}
