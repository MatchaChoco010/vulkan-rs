use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputImage {
    pub name: String,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl OutputImage {
    pub fn texel_size(&self) -> usize {
        match self.format.as_str() {
            "rgba8" | "d32f" | "r32ui" => 4,
            "s8" => 1,
            other => panic!("unknown output format {other}"),
        }
    }

    pub fn rgba8(&self, x: u32, y: u32) -> [u8; 4] {
        assert_eq!(self.format, "rgba8", "{} is not rgba8", self.name);
        let at = (y as usize * self.width as usize + x as usize) * 4;
        self.data[at..at + 4].try_into().unwrap()
    }

    pub fn word(&self, index: usize) -> u32 {
        u32::from_le_bytes(self.data[index * 4..index * 4 + 4].try_into().unwrap())
    }

    pub fn d32f(&self, x: u32, y: u32) -> f32 {
        assert_eq!(self.format, "d32f", "{} is not d32f", self.name);
        f32::from_bits(self.word(y as usize * self.width as usize + x as usize))
    }

    pub fn s8(&self, x: u32, y: u32) -> u8 {
        assert_eq!(self.format, "s8", "{} is not s8", self.name);
        self.data[y as usize * self.width as usize + x as usize]
    }

    pub fn is_picture(&self) -> bool {
        self.format != "r32ui"
    }

    pub fn to_rgb(&self) -> Vec<u8> {
        let texels = self.width as usize * self.height as usize;
        let mut rgb = Vec::with_capacity(texels * 3);
        for index in 0..texels {
            match self.format.as_str() {
                "rgba8" => rgb.extend_from_slice(&self.data[index * 4..index * 4 + 3]),
                "d32f" => {
                    let depth = f32::from_bits(self.word(index));
                    let gray = (depth.clamp(0.0, 1.0) * 255.0).round() as u8;
                    rgb.extend_from_slice(&[gray, gray, gray]);
                }
                "s8" => {
                    let gray = self.data[index].saturating_mul(85);
                    rgb.extend_from_slice(&[gray, gray, gray]);
                }
                other => panic!("{other} has no picture"),
            }
        }
        rgb
    }

    pub fn texels_differing(&self, other: &OutputImage) -> Vec<bool> {
        let size = self.texel_size();
        self.data
            .chunks_exact(size)
            .zip(other.data.chunks_exact(size))
            .map(|(a, b)| a != b)
            .collect()
    }
}

pub fn parse(path: &Path) -> Vec<OutputImage> {
    let bytes =
        std::fs::read(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    let magic = b"render-compare 1\n";
    assert!(
        bytes.starts_with(magic),
        "{} is not an output file",
        path.display()
    );
    let mut images = Vec::new();
    let mut rest = &bytes[magic.len()..];
    while !rest.is_empty() {
        let end = rest
            .iter()
            .position(|&byte| byte == b'\n')
            .expect("header line");
        let header = std::str::from_utf8(&rest[..end]).expect("UTF-8 header");
        let words = header.split(' ').collect::<Vec<_>>();
        assert!(
            words.len() == 6 && words[0] == "image",
            "{}: bad header {header:?}",
            path.display()
        );
        let size: usize = words[5].parse().expect("size");
        let data = rest[end + 1..end + 1 + size].to_vec();
        rest = &rest[end + 1 + size..];
        let image = OutputImage {
            name: words[1].to_string(),
            format: words[2].to_string(),
            width: words[3].parse().expect("width"),
            height: words[4].parse().expect("height"),
            data,
        };
        if image.is_picture() {
            assert_eq!(
                size,
                image.width as usize * image.height as usize * image.texel_size(),
                "{}: size of {}",
                path.display(),
                image.name
            );
        }
        images.push(image);
    }
    images
}
