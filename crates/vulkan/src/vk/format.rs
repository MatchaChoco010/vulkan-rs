//! Texel layout of each [`Format`], from the `<formats>` section of the registry.
//!
//! The Vulkan headers do not carry this information; applications otherwise keep their
//! own tables. [`Format::info`] is that table, generated.

use super::{Extent2D, Format, ImageAspectFlags};

/// What [`Format::info`] returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatInfo {
    /// Compatibility class, e.g. `"32-bit"`. Two formats of the same class can be copied
    /// between and, with `MUTABLE_FORMAT`, viewed as each other.
    pub class: &'static str,
    /// Size in bytes of one texel block. For multi-planar formats this is the sum over
    /// planes of one texel each; use the [`FormatPlane::compatible`] format for the
    /// layout of a single plane.
    pub block_size: u32,
    /// Number of texels in a texel block. 1 unless the format is block-compressed.
    pub texels_per_block: u32,
    /// Extent of a texel block in texels. `[1, 1, 1]` unless block-compressed.
    pub block_extent: [u32; 3],
    /// For `_PACKnn` formats, the width in bits of the word the components are packed in.
    pub packed: Option<u32>,
    /// Chroma subsampling of a Y'CbCr format.
    pub chroma: Option<ChromaSubsampling>,
    /// Compression scheme of a block-compressed format, e.g. `"BC"` or `"ASTC LDR"`.
    pub compression: Option<&'static str>,
    /// Components in the order the registry lists them, which for packed formats is
    /// from the most significant bits down.
    pub components: &'static [FormatComponent],
    /// Memory planes of a multi-planar format. Empty for single-plane formats.
    pub planes: &'static [FormatPlane],
}

/// One component of a format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatComponent {
    pub name: ComponentName,
    /// Width in bits, or `None` in a block-compressed format.
    pub bits: Option<u32>,
    pub numeric_format: NumericFormat,
    /// The plane that holds this component, in a multi-planar format.
    pub plane: Option<u32>,
}

/// One memory plane of a multi-planar format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatPlane {
    /// The width of the plane is the width of the image divided by this.
    pub width_divisor: u32,
    /// The height of the plane is the height of the image divided by this.
    pub height_divisor: u32,
    /// The single-plane format that has the layout of this plane.
    pub compatible: Format,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentName {
    R,
    G,
    B,
    A,
    /// Depth.
    D,
    /// Stencil.
    S,
}

/// How the bits of a component are interpreted.
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
    /// Unsigned normalized with the sRGB transfer function on R, G and B.
    Srgb,
    /// Signed fixed point with 5 fractional bits.
    Sfixed5,
    Bool,
}

/// The scalar type a shader sees when it reads a component: what the sampled type of an
/// image, or the component type of a color attachment output, has to be.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderNumericType {
    /// `float`: normalized, scaled, floating point and sRGB formats.
    Float,
    /// `int`
    Sint,
    /// `uint`
    Uint,
}

impl NumericFormat {
    /// The scalar type a shader reads this numeric format as. `None` for `Bool`, which
    /// has no image sampled type.
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

    /// `true` for the formats whose stored integers stand for a value in `[0, 1]` or
    /// `[-1, 1]`: `Unorm`, `Snorm` and `Srgb`.
    pub const fn is_normalized(self) -> bool {
        matches!(
            self,
            NumericFormat::Unorm | NumericFormat::Snorm | NumericFormat::Srgb
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChromaSubsampling {
    /// 4:2:0: chroma has half the width and half the height.
    _420,
    /// 4:2:2: chroma has half the width.
    _422,
    /// 4:4:4: chroma has the full resolution.
    _444,
}

impl FormatInfo {
    /// Number of memory planes: 1 unless the format is multi-planar.
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

    /// `true` if the color components are stored with the sRGB transfer function.
    pub fn is_srgb(&self) -> bool {
        self.components
            .iter()
            .any(|c| c.numeric_format == NumericFormat::Srgb)
    }

    /// The numeric format of the components, when they all share one. Color formats do;
    /// combined depth/stencil formats such as `D24_UNORM_S8_UINT` do not and give `None`.
    pub fn numeric_format(&self) -> Option<NumericFormat> {
        let first = self.components.first()?.numeric_format;
        self.components
            .iter()
            .all(|c| c.numeric_format == first)
            .then_some(first)
    }

    /// The scalar type a shader reads from `aspect` of an image of this format.
    ///
    /// `aspect` is `COLOR`, `DEPTH`, `STENCIL` or one `PLANE_n`; for a combined
    /// depth/stencil format the answer depends on it (`Float` for depth, `Uint` for
    /// stencil). `None` if the format does not have that aspect.
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

    /// The aspects an image of this format has: `DEPTH` and / or `STENCIL` for
    /// depth/stencil formats, `COLOR` for everything else, including multi-planar
    /// formats. Use [`FormatInfo::plane_aspect`] to address a single plane.
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

    /// The aspect that selects memory plane `plane` of a multi-planar format.
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

    /// The extent of memory plane `plane` of an image with extent `extent`. For a
    /// single-plane format, plane 0 has the extent of the image.
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
