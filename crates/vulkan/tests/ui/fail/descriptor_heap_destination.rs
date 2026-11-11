//! The host memory a descriptor is written to must outlive the range that names it.
use vulkan::vk;

fn main() {
    let destination;
    {
        let mut heap_bytes = [0u8; 64];
        destination = vk::HostAddressRangeEXT::default().address(&mut heap_bytes);
    }
    let _ = destination;
}
