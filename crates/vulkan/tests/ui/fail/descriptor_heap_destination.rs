use vulkan::vk;

fn main() {
    let destination;
    {
        let mut heap_bytes = [0u8; 64];
        destination = vk::HostAddressRangeEXT::default().address(&mut heap_bytes);
    }
    let _ = destination;
}
