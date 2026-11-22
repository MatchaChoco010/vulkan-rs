use super::{Extent2D, Format, ImageAspectFlags};

/// The texel layout of a [`Format`], from the `<formats>` section of the registry, as
/// returned by [`Format::info`].
///
/// ```
/// use vulkan::vk::{self, ComponentName, NumericFormat};
///
/// let rgba = vk::Format::R8G8B8A8_UNORM.info().unwrap();
/// assert_eq!(rgba.class, "32-bit");
/// assert_eq!(rgba.block_size, 4);
/// assert_eq!(rgba.components[3].name, ComponentName::A);
/// assert_eq!(rgba.numeric_format(), Some(NumericFormat::Unorm));
/// assert_eq!(rgba.aspects(), vk::ImageAspectFlags::COLOR);
///
/// let nv12 = vk::Format::G8_B8R8_2PLANE_420_UNORM.info().unwrap();
/// assert_eq!(nv12.plane_count(), 2);
/// assert_eq!(nv12.planes[1].compatible, vk::Format::R8G8_UNORM);
/// let extent = vk::Extent2D { width: 1920, height: 1080 };
/// let chroma = nv12.plane_extent(1, extent).unwrap();
/// assert_eq!((chroma.width, chroma.height), (960, 540));
///
/// assert!(vk::Format::UNDEFINED.info().is_none());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatInfo {
    /// The format compatibility class, for example `"32-bit"` or `"BC1_RGB"`. Formats of
    /// one class are compatible in the sense of the specification, for example for image
    /// views of an image created with `MUTABLE_FORMAT`.
    pub class: &'static str,
    /// The size in bytes of one texel block. For a multi-planar format this is the sum
    /// over the planes of one texel each; the [`FormatPlane::compatible`] format gives the
    /// layout of a single plane.
    pub block_size: u32,
    /// The number of texels in a texel block, as the registry gives it: 1 for formats that
    /// are not block-compressed. The registry also gives 1 for the PVRTC formats; use
    /// [`block_extent`](FormatInfo::block_extent) for their block size in texels.
    pub texels_per_block: u32,
    /// The extent of a texel block in texels: `[1, 1, 1]` unless the format is
    /// block-compressed or has a 4:2:2 single-plane layout.
    pub block_extent: [u32; 3],
    /// For a `_PACKnn` format, the width in bits of the word the components are packed in.
    pub packed: Option<u32>,
    /// The chroma subsampling of a Y'CbCr format.
    pub chroma: Option<ChromaSubsampling>,
    /// The compression scheme of a block-compressed format: `"BC"`, `"ETC2"`, `"EAC"`,
    /// `"ASTC LDR"`, `"ASTC HDR"` or `"PVRTC"`.
    pub compression: Option<&'static str>,
    /// The components in the order the registry lists them, which for packed formats is
    /// from the most significant bits down.
    pub components: &'static [FormatComponent],
    /// The memory planes of a multi-planar format. Empty for single-plane formats.
    pub planes: &'static [FormatPlane],
}

/// One component of a format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatComponent {
    /// Which component this is.
    pub name: ComponentName,
    /// The width in bits. `None` in a block-compressed format.
    pub bits: Option<u32>,
    /// How the bits are interpreted.
    pub numeric_format: NumericFormat,
    /// The index of the plane that holds this component, in a multi-planar format.
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

/// The name of a format component. In Y'CbCr formats, `G` is Y', `B` is Cb and `R` is Cr.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentName {
    /// Red, or Cr.
    R,
    /// Green, or Y'.
    G,
    /// Blue, or Cb.
    B,
    /// Alpha.
    A,
    /// Depth.
    D,
    /// Stencil.
    S,
}

/// How the bits of a component are interpreted, the numeric format part of a format name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NumericFormat {
    /// Unsigned normalized: read as a float in `[0, 1]`.
    Unorm,
    /// Signed normalized: read as a float in `[-1, 1]`.
    Snorm,
    /// Unsigned integer, read as a float with the same value.
    Uscaled,
    /// Signed integer, read as a float with the same value.
    Sscaled,
    /// Unsigned integer.
    Uint,
    /// Signed integer.
    Sint,
    /// Unsigned floating point.
    Ufloat,
    /// Signed floating point.
    Sfloat,
    /// Unsigned normalized with the sRGB transfer function on R, G and B.
    Srgb,
    /// Signed fixed point with 5 fractional bits.
    Sfixed5,
    /// Boolean.
    Bool,
}

/// The scalar type a shader sees when it reads a component: the type the sampled type of
/// an image, or the component type of a color attachment output, has to have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderNumericType {
    /// Floating point: normalized, scaled, floating point, sRGB and fixed point formats.
    Float,
    /// Signed integer.
    Sint,
    /// Unsigned integer.
    Uint,
}

impl NumericFormat {
    /// The scalar type a shader reads this numeric format as. `None` for
    /// [`Bool`](NumericFormat::Bool).
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

    /// Whether the stored integers stand for values in `[0, 1]` or `[-1, 1]`: `true` for
    /// [`Unorm`](NumericFormat::Unorm), [`Snorm`](NumericFormat::Snorm) and
    /// [`Srgb`](NumericFormat::Srgb).
    pub const fn is_normalized(self) -> bool {
        matches!(
            self,
            NumericFormat::Unorm | NumericFormat::Snorm | NumericFormat::Srgb
        )
    }
}

/// The chroma subsampling of a Y'CbCr format.
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
    /// The number of memory planes: 1 unless the format is multi-planar.
    pub const fn plane_count(&self) -> u32 {
        if self.planes.is_empty() {
            1
        } else {
            self.planes.len() as u32
        }
    }

    /// Whether the format has more than one memory plane.
    pub const fn is_multiplanar(&self) -> bool {
        !self.planes.is_empty()
    }

    /// Whether the format is block-compressed.
    pub const fn is_compressed(&self) -> bool {
        self.compression.is_some()
    }

    /// Whether the format has a depth component.
    pub fn has_depth(&self) -> bool {
        self.has_component(ComponentName::D)
    }

    /// Whether the format has a stencil component.
    pub fn has_stencil(&self) -> bool {
        self.has_component(ComponentName::S)
    }

    /// Whether the format has an alpha component.
    pub fn has_alpha(&self) -> bool {
        self.has_component(ComponentName::A)
    }

    /// Whether the format has the component `name`.
    pub fn has_component(&self, name: ComponentName) -> bool {
        self.components.iter().any(|c| c.name == name)
    }

    /// Whether the color components are stored with the sRGB transfer function.
    pub fn is_srgb(&self) -> bool {
        self.components
            .iter()
            .any(|c| c.numeric_format == NumericFormat::Srgb)
    }

    /// The numeric format of the components, if they all share one. Color formats do;
    /// combined depth/stencil formats such as `D24_UNORM_S8_UINT` do not and give `None`.
    pub fn numeric_format(&self) -> Option<NumericFormat> {
        let first = self.components.first()?.numeric_format;
        self.components
            .iter()
            .all(|c| c.numeric_format == first)
            .then_some(first)
    }

    /// The scalar type a shader reads from the aspect `aspect` of an image of this format.
    ///
    /// `aspect` must be exactly one of `COLOR`, `DEPTH`, `STENCIL`, `PLANE_0`, `PLANE_1` and
    /// `PLANE_2`. For a combined depth/stencil format the answer depends on it (`Float`
    /// for depth, `Uint` for stencil). Returns `None` if the format does not have that
    /// aspect, for any other value of `aspect`, and for [`Bool`](NumericFormat::Bool)
    /// components.
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

    /// The aspects of an image of this format: `DEPTH` and/or `STENCIL` for depth/stencil
    /// formats, `COLOR` for every other format, multi-planar formats included. Use
    /// [`FormatInfo::plane_aspect`] to address a single plane.
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

    /// The aspect (`PLANE_0`, `PLANE_1` or `PLANE_2`) that selects memory plane `plane` of
    /// a multi-planar format. `None` for single-plane formats and if `plane` is not less
    /// than the number of planes.
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

    /// The extent of memory plane `plane` of an image with extent `extent`, the image
    /// extent divided (rounding down) by the divisors of the plane. For a single-plane
    /// format, plane 0 has the extent of the image. `None` if the format has no plane
    /// `plane`.
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
