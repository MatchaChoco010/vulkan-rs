//! A descriptor to be written into a heap borrows through a union and two levels of
//! structs: the image view description must outlive the descriptor info.
use vulkan::vk;

fn main() {
    let descriptor;
    {
        let view = vk::ImageViewCreateInfo::default();
        let image = vk::ImageDescriptorInfoEXT::default().view(&view);
        descriptor = vk::ResourceDescriptorInfoEXT::default()
            .data(vk::ResourceDescriptorDataEXT::image(&image));
    }
    let _ = descriptor;
}
