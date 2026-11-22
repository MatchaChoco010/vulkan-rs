//! Exports a rendered image as a dma-buf and imports it again, the way a Wayland client
//! and a compositor share buffers.
//!
//! 1. Client side (`export.rs`): create an image with a DRM format modifier, render into
//!    it, and describe it the way `zwp_linux_dmabuf_v1` does: fd, format, modifier, and
//!    the offset and stride of each plane.
//! 2. Compositor side (`import.rs`): check that the description can be imported, create
//!    an image with exactly that layout, import the fd as its memory, and read the
//!    pixels back.
//!
//! Both sides run on one device here. Nothing in the import path depends on that.
//!
//! The image goes to `target/example-output/dmabuf_roundtrip.ppm`.

mod export;
mod gpu;
mod import;

use anyhow::{Result, bail};
use std::os::fd::OwnedFd;
use vulkan::vk;
use vulkan_example_common::output::{output_path, write_ppm};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
/// `DRM_FORMAT_ARGB8888` in DRM terms.
const FORMAT: vk::Format = vk::Format::B8G8R8A8_UNORM;
const DMA_BUF: vk::ExternalMemoryHandleTypeFlagBits =
    vk::ExternalMemoryHandleTypeFlagBits::DMA_BUF_EXT;

const BACKGROUND: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
const RECTANGLE: [f32; 4] = [0.0, 0.25, 1.0, 1.0];

/// What a client sends to a compositor for one buffer.
struct Dmabuf {
    fd: OwnedFd,
    width: u32,
    height: u32,
    format: vk::Format,
    modifier: u64,
    planes: Vec<Plane>,
}

/// One memory plane of a dma-buf, as `zwp_linux_buffer_params_v1.add` describes it.
#[derive(Copy, Clone)]
struct Plane {
    offset: u64,
    stride: u64,
}

fn main() -> Result<()> {
    // SAFETY: the example upholds the valid usage rules of the commands it calls; the
    // validation layer, when installed, reports any violation.
    unsafe { run() }
}

unsafe fn run() -> Result<()> {
    unsafe {
        // Every object below is destroyed when it goes out of scope, in the reverse order
        // of creation, also when a step fails and `?` returns early. The client's image
        // outlives the compositor's import of its memory.
        let gpu = gpu::create_gpu()?;
        gpu::print_drm_nodes(&gpu)?;

        let (_client_image, dmabuf) = export::render_and_export(&gpu)?;
        println!(
            "exported {}x{} {:?}, modifier {:#018x}, {} plane(s), offset {} stride {}",
            dmabuf.width,
            dmabuf.height,
            dmabuf.format,
            dmabuf.modifier,
            dmabuf.planes.len(),
            dmabuf.planes[0].offset,
            dmabuf.planes[0].stride,
        );

        let extent = vk::Extent2D {
            width: dmabuf.width,
            height: dmabuf.height,
        };
        let compositor_image = import::import_image(&gpu, dmabuf)?;
        let pixels = import::read_back(&gpu, &compositor_image, extent)?;
        verify(&pixels, extent)?;
        let path = output_path("dmabuf_roundtrip.ppm")?;
        write_ppm(
            &path,
            extent.width,
            extent.height,
            pixels
                .chunks_exact(4)
                .map(|bgra| [bgra[2], bgra[1], bgra[0]]),
        )?;
        println!("imported pixels match; wrote {}", path.display());
        Ok(())
    }
}

/// Checks that the imported image holds what the exporter drew.
fn verify(bgra: &[u8], extent: vk::Extent2D) -> Result<()> {
    let vk::Extent2D { width, height } = extent;
    let expect = |rgba: [f32; 4]| {
        let byte = |value: f32| (value * 255.0).round() as u8;
        [byte(rgba[2]), byte(rgba[1]), byte(rgba[0]), byte(rgba[3])]
    };
    for (x, y, color) in [
        (width / 4, height / 4, BACKGROUND),
        (width * 3 / 4, height / 4, BACKGROUND),
        (width / 4, height * 3 / 4, BACKGROUND),
        (width * 3 / 4, height * 3 / 4, RECTANGLE),
    ] {
        let index = ((y * width + x) * 4) as usize;
        let pixel = &bgra[index..index + 4];
        let expected = expect(color);
        // Allow one step of rounding difference between implementations.
        if pixel.iter().zip(expected).any(|(a, b)| a.abs_diff(b) > 1) {
            bail!("pixel ({x}, {y}) is {pixel:?}, expected {expected:?} (BGRA)");
        }
    }
    Ok(())
}
