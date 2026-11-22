use super::{Extent2D, Format, ImageAspectFlags};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatInfo {
    pub class: &'static str,
    pub block_size: u32,
    pub texels_per_block: u32,
    pub block_extent: [u32; 3],
    pub packed: Option<u32>,
    pub chroma: Option<ChromaSubsampling>,
    pub compression: Option<&'static str>,
    pub components: &'static [FormatComponent],
    pub planes: &'static [FormatPlane],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatComponent {
    pub name: ComponentName,
    pub bits: Option<u32>,
    pub numeric_format: NumericFormat,
    pub plane: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatPlane {
    pub width_divisor: u32,
    pub height_divisor: u32,
    pub compatible: Format,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentName {
    R,
    G,
    B,
    A,
    D,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NumericFormat {
    Unorm,
    Snorm,
    Uscaled,
    Sscaled,
    Uint,
    Sint,
    Ufloat,
    Sfloat,
    Srgb,
    Sfixed5,
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderNumericType {
    Float,
    Sint,
    Uint,
}

impl NumericFormat {
    pub const fn shader_type(self) -> Option<ShaderNumericType> {
        match self {
            NumericFormat::Unorm
            | NumericFormat::Snorm
            | NumericFormat::Uscaled
            | NumericFormat::Sscaled
            | NumericFormat::Ufloat
            | NumericFormat::Sfloat
            | NumericFormat::Srgb
            | NumericFormat::Sfixed5 => Some(ShaderNumericType::Float),
            NumericFormat::Sint => Some(ShaderNumericType::Sint),
            NumericFormat::Uint => Some(ShaderNumericType::Uint),
            NumericFormat::Bool => None,
        }
    }

    pub const fn is_normalized(self) -> bool {
        matches!(
            self,
            NumericFormat::Unorm | NumericFormat::Snorm | NumericFormat::Srgb
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChromaSubsampling {
    _420,
    _422,
    _444,
}

impl FormatInfo {
    pub const fn plane_count(&self) -> u32 {
        if self.planes.is_empty() {
            1
        } else {
            self.planes.len() as u32
        }
    }

    pub const fn is_multiplanar(&self) -> bool {
        !self.planes.is_empty()
    }

    pub const fn is_compressed(&self) -> bool {
        self.compression.is_some()
    }

    pub fn has_depth(&self) -> bool {
        self.has_component(ComponentName::D)
    }

    pub fn has_stencil(&self) -> bool {
        self.has_component(ComponentName::S)
    }

    pub fn has_alpha(&self) -> bool {
        self.has_component(ComponentName::A)
    }

    pub fn has_component(&self, name: ComponentName) -> bool {
        self.components.iter().any(|c| c.name == name)
    }

    pub fn is_srgb(&self) -> bool {
        self.components
            .iter()
            .any(|c| c.numeric_format == NumericFormat::Srgb)
    }

    pub fn numeric_format(&self) -> Option<NumericFormat> {
        let first = self.components.first()?.numeric_format;
        self.components
            .iter()
            .all(|c| c.numeric_format == first)
            .then_some(first)
    }

    pub fn shader_type(&self, aspect: ImageAspectFlags) -> Option<ShaderNumericType> {
        let wanted = |c: &&FormatComponent| {
            if aspect == ImageAspectFlags::DEPTH {
                c.name == ComponentName::D
            } else if aspect == ImageAspectFlags::STENCIL {
                c.name == ComponentName::S
            } else if aspect == ImageAspectFlags::COLOR {
                !matches!(c.name, ComponentName::D | ComponentName::S)
            } else if aspect == ImageAspectFlags::PLANE_0 {
                c.plane == Some(0)
            } else if aspect == ImageAspectFlags::PLANE_1 {
                c.plane == Some(1)
            } else if aspect == ImageAspectFlags::PLANE_2 {
                c.plane == Some(2)
            } else {
                false
            }
        };
        self.components
            .iter()
            .find(wanted)
            .and_then(|c| c.numeric_format.shader_type())
    }

    pub fn aspects(&self) -> ImageAspectFlags {
        let mut aspects = ImageAspectFlags::empty();
        if self.has_depth() {
            aspects |= ImageAspectFlags::DEPTH;
        }
        if self.has_stencil() {
            aspects |= ImageAspectFlags::STENCIL;
        }
        if aspects.is_empty() {
            aspects = ImageAspectFlags::COLOR;
        }
        aspects
    }

    pub fn plane_aspect(&self, plane: u32) -> Option<ImageAspectFlags> {
        if plane as usize >= self.planes.len() {
            return None;
        }
        match plane {
            0 => Some(ImageAspectFlags::PLANE_0),
            1 => Some(ImageAspectFlags::PLANE_1),
            2 => Some(ImageAspectFlags::PLANE_2),
            _ => None,
        }
    }

    pub fn plane_extent(&self, plane: u32, extent: Extent2D) -> Option<Extent2D> {
        if self.planes.is_empty() {
            return (plane == 0).then_some(extent);
        }
        let plane = self.planes.get(plane as usize)?;
        Some(Extent2D {
            width: extent.width / plane.width_divisor,
            height: extent.height / plane.height_divisor,
        })
    }
}
