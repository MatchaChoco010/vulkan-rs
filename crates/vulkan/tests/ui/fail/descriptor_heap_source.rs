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
