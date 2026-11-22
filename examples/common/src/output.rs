use anyhow::{Context, Result, ensure};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

/// Where an example writes `file_name`: `target/example-output/` of the workspace, never
/// the source tree. The directory is created if it does not exist.
pub fn output_path(file_name: &str) -> Result<PathBuf> {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .context("examples/common has a workspace root two levels up")?;
    let directory = workspace.join("target/example-output");
    std::fs::create_dir_all(&directory)
        .with_context(|| format!("creating {}", directory.display()))?;
    Ok(directory.join(file_name))
}

/// Writes a binary PPM image, a format that needs no library to write and that most
/// image viewers open. `pixels` are RGB, row by row from the top.
pub fn write_ppm(
    path: &Path,
    width: u32,
    height: u32,
    pixels: impl IntoIterator<Item = [u8; 3]>,
) -> Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    write!(file, "P6\n{width} {height}\n255\n")?;
    let mut count = 0_u64;
    for pixel in pixels {
        file.write_all(&pixel)?;
        count += 1;
    }
    ensure!(
        count == u64::from(width) * u64::from(height),
        "{count} pixels for a {width}x{height} image"
    );
    file.flush()?;
    Ok(())
}

/// [`write_ppm`] for tightly packed RGBA8 pixels; the alpha channel is dropped.
pub fn write_rgba_ppm(path: &Path, width: u32, height: u32, rgba: &[u8]) -> Result<()> {
    write_ppm(
        path,
        width,
        height,
        rgba.chunks_exact(4)
            .map(|pixel| [pixel[0], pixel[1], pixel[2]]),
    )
}
