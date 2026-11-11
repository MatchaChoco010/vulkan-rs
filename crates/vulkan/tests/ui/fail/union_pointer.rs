//! A union that selects a pointer member borrows what it points to.
use vulkan::vk;

fn main() {
    let info;
    {
        let image = vk::DescriptorImageInfo::default();
        let data = vk::DescriptorDataEXT::sampled_image(&image);
        info = vk::DescriptorGetInfoEXT::default().data(data);
    }
    let _ = info;
}
