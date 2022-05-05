//!
//! Contain texture data types.
//!

pub use half::f16;

///
/// Possible modes of interpolation which determines the texture output between texture pixels.
///
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Interpolation {
    Nearest,
    Linear,
}

///
/// Possible wrapping modes for a texture which determines how the texture is applied outside of the
/// [0..1] uv coordinate range.
///
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Wrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
}

///
/// The pixel/texel data for a [Texture2D] or [Texture3D].
///
/// If 2D data, the data array should start with the top left texel and then one row at a time.
/// The indices `(row, column)` into the 2D data would look like
/// ```notrust
/// [
/// (0, 0), (1, 0), .., // First row
/// (0, 1), (1, 1), .., // Second row
/// ..
/// ]
/// ```
/// If 3D data, the data array would look like the 2D data, one layer/image at a time.
/// The indices `(row, column, layer)` into the 3D data would look like
/// ```notrust
/// [
/// (0, 0, 0), (1, 0, 0), .., // First row in first layer
/// (0, 1, 0), (1, 1, 0), .., // Second row in first layer
/// ..
/// (0, 0, 1), (1, 0, 1), .., // First row in second layer
/// (0, 1, 1), (1, 1, 1), ..,  // Second row in second layer
/// ..
/// ]
/// ```
///
#[derive(Clone)]
pub enum TextureData {
    /// One byte in the red channel.
    RU8(Vec<u8>),
    /// One byte in the red and green channel.
    RgU8(Vec<[u8; 2]>),
    /// One byte in the red, green and blue channel.
    RgbU8(Vec<[u8; 3]>),
    /// One byte in the red, green, blue and alpha channel.
    RgbaU8(Vec<[u8; 4]>),

    /// 16-bit float in the red channel.
    RF16(Vec<f16>),
    /// 16-bit float in the red and green channel.
    RgF16(Vec<[f16; 2]>),
    /// 16-bit float in the red, green and blue channel.
    RgbF16(Vec<[f16; 3]>),
    /// 16-bit float in the red, green, blue and alpha channel.
    RgbaF16(Vec<[f16; 4]>),

    /// 32-bit float in the red channel.
    RF32(Vec<f32>),
    /// 32-bit float in the red and green channel.
    RgF32(Vec<[f32; 2]>),
    /// 32-bit float in the red, green and blue channel.
    RgbF32(Vec<[f32; 3]>),
    /// 32-bit float in the red, green, blue and alpha channel.
    RgbaF32(Vec<[f32; 4]>),
}

impl std::fmt::Debug for TextureData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RU8(values) => write!(f, "R u8 ({:?})", values.len()),
            Self::RgU8(values) => write!(f, "RG u8 ({:?})", values.len()),
            Self::RgbU8(values) => write!(f, "RGB u8 ({:?})", values.len()),
            Self::RgbaU8(values) => write!(f, "RGBA u8 ({:?})", values.len()),
            Self::RF16(values) => write!(f, "R f16 ({:?})", values.len()),
            Self::RgF16(values) => write!(f, "RG f16 ({:?})", values.len()),
            Self::RgbF16(values) => write!(f, "RGB f16 ({:?})", values.len()),
            Self::RgbaF16(values) => write!(f, "RGBA f16 ({:?})", values.len()),
            Self::RF32(values) => write!(f, "R f32 ({:?})", values.len()),
            Self::RgF32(values) => write!(f, "RG f32 ({:?})", values.len()),
            Self::RgbF32(values) => write!(f, "RGB f32 ({:?})", values.len()),
            Self::RgbaF32(values) => write!(f, "RGBA f32 ({:?})", values.len()),
        }
    }
}

///
/// A CPU-side version of a 2D texture.
///
#[derive(Clone, Debug)]
pub struct Texture2D {
    /// The pixel data for the image
    pub data: TextureData,
    /// The width of the image
    pub width: u32,
    /// The height of the image
    pub height: u32,
    /// The way the pixel data is interpolated when the texture is far away
    pub min_filter: Interpolation,
    /// The way the pixel data is interpolated when the texture is close
    pub mag_filter: Interpolation,
    /// Specifies whether mipmaps should be created for this texture and what type of interpolation to use between the two closest mipmaps.
    /// Note, however, that the mipmaps only will be created if the width and height of the texture are power of two.
    pub mip_map_filter: Option<Interpolation>,
    /// Determines how the texture is sampled outside the [0..1] s coordinate range (the first value of the uv coordinates).
    pub wrap_s: Wrapping,
    /// Determines how the texture is sampled outside the [0..1] t coordinate range (the second value of the uv coordinates).
    pub wrap_t: Wrapping,
}

impl Default for Texture2D {
    fn default() -> Self {
        Self {
            data: TextureData::RgbaU8(vec![[0, 0, 0, 0]]),
            width: 1,
            height: 1,
            min_filter: Interpolation::Linear,
            mag_filter: Interpolation::Linear,
            mip_map_filter: Some(Interpolation::Linear),
            wrap_s: Wrapping::Repeat,
            wrap_t: Wrapping::Repeat,
        }
    }
}

///
/// A CPU-side version of a [Texture3D].
///
#[derive(Clone, Debug)]
pub struct Texture3D {
    /// The pixel data for the image
    pub data: TextureData,
    /// The width of the image
    pub width: u32,
    /// The height of the image
    pub height: u32,
    /// The depth of the image
    pub depth: u32,
    /// The way the pixel data is interpolated when the texture is far away
    pub min_filter: Interpolation,
    /// The way the pixel data is interpolated when the texture is close
    pub mag_filter: Interpolation,
    /// Specifies whether mipmaps should be created for this texture and what type of interpolation to use between the two closest mipmaps.
    /// Note, however, that the mipmaps only will be created if the width and height of the texture are power of two.
    pub mip_map_filter: Option<Interpolation>,
    /// Determines how the texture is sampled outside the [0..1] s coordinate range (the first value of the uvw coordinates).
    pub wrap_s: Wrapping,
    /// Determines how the texture is sampled outside the [0..1] t coordinate range (the second value of the uvw coordinates).
    pub wrap_t: Wrapping,
    /// Determines how the texture is sampled outside the [0..1] r coordinate range (the third value of the uvw coordinates).
    pub wrap_r: Wrapping,
}

impl Default for Texture3D {
    fn default() -> Self {
        Self {
            data: TextureData::RgbaU8(vec![[0, 0, 0, 0]]),
            width: 1,
            height: 1,
            depth: 1,
            min_filter: Interpolation::Linear,
            mag_filter: Interpolation::Linear,
            mip_map_filter: Some(Interpolation::Linear),
            wrap_s: Wrapping::Repeat,
            wrap_t: Wrapping::Repeat,
            wrap_r: Wrapping::Repeat,
        }
    }
}

///
/// The pixel data for a [TextureCube].
///
#[derive(Clone)]
pub enum TextureCubeData {
    /// byte in the red channel.
    RU8(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>),
    /// byte in the red and green channel.
    RgU8(
        Vec<[u8; 2]>,
        Vec<[u8; 2]>,
        Vec<[u8; 2]>,
        Vec<[u8; 2]>,
        Vec<[u8; 2]>,
        Vec<[u8; 2]>,
    ),
    /// byte in the red, green and blue channel.
    RgbU8(
        Vec<[u8; 3]>,
        Vec<[u8; 3]>,
        Vec<[u8; 3]>,
        Vec<[u8; 3]>,
        Vec<[u8; 3]>,
        Vec<[u8; 3]>,
    ),
    /// byte in the red, green, blue and alpha channel.
    RgbaU8(
        Vec<[u8; 4]>,
        Vec<[u8; 4]>,
        Vec<[u8; 4]>,
        Vec<[u8; 4]>,
        Vec<[u8; 4]>,
        Vec<[u8; 4]>,
    ),

    /// 16-bit float in the red channel.
    RF16(Vec<f16>, Vec<f16>, Vec<f16>, Vec<f16>, Vec<f16>, Vec<f16>),
    /// 16-bit float in the red and green channel.
    RgF16(
        Vec<[f16; 2]>,
        Vec<[f16; 2]>,
        Vec<[f16; 2]>,
        Vec<[f16; 2]>,
        Vec<[f16; 2]>,
        Vec<[f16; 2]>,
    ),
    /// 16-bit float in the red, green and blue channel.
    RgbF16(
        Vec<[f16; 3]>,
        Vec<[f16; 3]>,
        Vec<[f16; 3]>,
        Vec<[f16; 3]>,
        Vec<[f16; 3]>,
        Vec<[f16; 3]>,
    ),
    /// 16-bit float in the red, green, blue and alpha channel.
    RgbaF16(
        Vec<[f16; 4]>,
        Vec<[f16; 4]>,
        Vec<[f16; 4]>,
        Vec<[f16; 4]>,
        Vec<[f16; 4]>,
        Vec<[f16; 4]>,
    ),

    /// 32-bit float in the red channel.
    RF32(Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>),
    /// 32-bit float in the red and green channel.
    RgF32(
        Vec<[f32; 2]>,
        Vec<[f32; 2]>,
        Vec<[f32; 2]>,
        Vec<[f32; 2]>,
        Vec<[f32; 2]>,
        Vec<[f32; 2]>,
    ),
    /// 32-bit float in the red, green and blue channel.
    RgbF32(
        Vec<[f32; 3]>,
        Vec<[f32; 3]>,
        Vec<[f32; 3]>,
        Vec<[f32; 3]>,
        Vec<[f32; 3]>,
        Vec<[f32; 3]>,
    ),
    /// 32-bit float in the red, green, blue and alpha channel.
    RgbaF32(
        Vec<[f32; 4]>,
        Vec<[f32; 4]>,
        Vec<[f32; 4]>,
        Vec<[f32; 4]>,
        Vec<[f32; 4]>,
        Vec<[f32; 4]>,
    ),
}

///
/// A CPU-side version of a cube map texture. All 6 images must have the same dimensions.
///
pub struct TextureCube {
    /// The pixel data for the cube image
    pub data: TextureCubeData,
    /// The width of each of the 6 images
    pub width: u32,
    /// The height of each of the 6 images
    pub height: u32,
    /// The way the pixel data is interpolated when the texture is far away
    pub min_filter: Interpolation,
    /// The way the pixel data is interpolated when the texture is close
    pub mag_filter: Interpolation,
    /// Specifies whether mipmaps should be created for this texture and what type of interpolation to use between the two closest mipmaps.
    /// Note, however, that the mipmaps only will be created if the width and height of the texture are power of two.
    pub mip_map_filter: Option<Interpolation>,
    /// Determines how the texture is sampled outside the [0..1] s coordinate range.
    pub wrap_s: Wrapping,
    /// Determines how the texture is sampled outside the [0..1] t coordinate range.
    pub wrap_t: Wrapping,
    /// Determines how the texture is sampled outside the [0..1] r coordinate range.
    pub wrap_r: Wrapping,
}

impl Default for TextureCube {
    fn default() -> Self {
        Self {
            data: TextureCubeData::RgbaU8(
                vec![[255, 0, 0, 255]],
                vec![[255, 0, 0, 255]],
                vec![[255, 0, 0, 255]],
                vec![[255, 0, 0, 255]],
                vec![[255, 0, 0, 255]],
                vec![[255, 0, 0, 255]],
            ),
            width: 1,
            height: 1,
            min_filter: Interpolation::Linear,
            mag_filter: Interpolation::Linear,
            mip_map_filter: Some(Interpolation::Linear),
            wrap_s: Wrapping::Repeat,
            wrap_t: Wrapping::Repeat,
            wrap_r: Wrapping::Repeat,
        }
    }
}

impl std::fmt::Debug for TextureCube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextureCube")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("min_filter", &self.min_filter)
            .field("mag_filter", &self.mag_filter)
            .field("mip_map_filter", &self.mip_map_filter)
            .field("wrap_s", &self.wrap_s)
            .field("wrap_t", &self.wrap_t)
            .field("wrap_r", &self.wrap_r)
            .finish()
    }
}
