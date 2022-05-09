use crate::{io::Deserialize, io::Loaded, io::Serialize, texture::*, Result};
use image::{io::Reader, *};
use std::io::Cursor;
use std::path::Path;

impl Deserialize for Texture2D {
    fn deserialize(bytes: &[u8]) -> Result<Self> {
        let reader = Reader::new(Cursor::new(bytes))
            .with_guessed_format()
            .expect("Cursor io never fails");
        #[cfg(feature = "hdr")]
        if reader.format() == Some(image::ImageFormat::Hdr) {
            use image::codecs::hdr::*;
            let decoder = HdrDecoder::new(bytes)?;
            let metadata = decoder.metadata();
            let img = decoder.read_image_native()?;
            return Ok(Texture2D {
                data: TextureData::RgbF32(
                    img.iter()
                        .map(|rgbe| {
                            let Rgb(values) = rgbe.to_hdr();
                            [values[0], values[1], values[2]]
                        })
                        .collect::<Vec<_>>(),
                ),
                width: metadata.width,
                height: metadata.height,
                ..Default::default()
            });
        }
        let img: DynamicImage = reader.decode()?;
        let width = img.width();
        let height = img.height();
        let data = match img {
            DynamicImage::ImageLuma8(_) => TextureData::RU8(img.into_bytes()),
            DynamicImage::ImageLumaA8(_) => {
                let bytes = img.as_bytes();
                let mut data = Vec::new();
                for i in 0..bytes.len() / 2 {
                    data.push([bytes[i * 2], bytes[i * 2 + 1]]);
                }
                TextureData::RgU8(data)
            }
            DynamicImage::ImageRgb8(_) => {
                let bytes = img.as_bytes();
                let mut data = Vec::new();
                for i in 0..bytes.len() / 3 {
                    data.push([bytes[i * 3], bytes[i * 3 + 1], bytes[i * 3 + 2]]);
                }
                TextureData::RgbU8(data)
            }
            DynamicImage::ImageRgba8(_) => {
                let bytes = img.as_bytes();
                let mut data = Vec::new();
                for i in 0..bytes.len() / 4 {
                    data.push([
                        bytes[i * 4],
                        bytes[i * 4 + 1],
                        bytes[i * 4 + 2],
                        bytes[i * 4 + 3],
                    ]);
                }
                TextureData::RgbaU8(data)
            }
            _ => unimplemented!(),
        };
        Ok(Self {
            data,
            width,
            height,
            ..Default::default()
        })
    }
}

impl Serialize for Texture2D {
    fn serialize(&self) -> Result<Vec<u8>> {
        // TODO: Put actual pixel data
        let img = match &self.data {
            TextureData::RgbaU8(data) => DynamicImage::new_rgba8(self.width, self.height),
            _ => unimplemented!(),
        };
        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
        Ok(bytes)
    }
}

impl TextureCube {
    ///
    /// Deserialize the 6 images given as byte arrays into a [TextureCube].
    ///
    pub fn from_bytes(
        right_bytes: &[u8],
        left_bytes: &[u8],
        top_bytes: &[u8],
        bottom_bytes: &[u8],
        front_bytes: &[u8],
        back_bytes: &[u8],
    ) -> Result<Self> {
        let right = Texture2D::deserialize(right_bytes)?;
        let left = Texture2D::deserialize(left_bytes)?;
        let top = Texture2D::deserialize(top_bytes)?;
        let bottom = Texture2D::deserialize(bottom_bytes)?;
        let front = Texture2D::deserialize(front_bytes)?;
        let back = Texture2D::deserialize(back_bytes)?;
        let data = match right.data {
            TextureData::RU8(right) => {
                let left = if let TextureData::RU8(data) = left.data {
                    data
                } else {
                    unreachable!()
                };
                let top = if let TextureData::RU8(data) = top.data {
                    data
                } else {
                    unreachable!()
                };
                let bottom = if let TextureData::RU8(data) = bottom.data {
                    data
                } else {
                    unreachable!()
                };
                let front = if let TextureData::RU8(data) = front.data {
                    data
                } else {
                    unreachable!()
                };
                let back = if let TextureData::RU8(data) = back.data {
                    data
                } else {
                    unreachable!()
                };
                TextureCubeData::RU8(right, left, top, bottom, front, back)
            }
            TextureData::RgU8(right) => {
                let left = if let TextureData::RgU8(data) = left.data {
                    data
                } else {
                    unreachable!()
                };
                let top = if let TextureData::RgU8(data) = top.data {
                    data
                } else {
                    unreachable!()
                };
                let bottom = if let TextureData::RgU8(data) = bottom.data {
                    data
                } else {
                    unreachable!()
                };
                let front = if let TextureData::RgU8(data) = front.data {
                    data
                } else {
                    unreachable!()
                };
                let back = if let TextureData::RgU8(data) = back.data {
                    data
                } else {
                    unreachable!()
                };
                TextureCubeData::RgU8(right, left, top, bottom, front, back)
            }
            TextureData::RgbU8(right) => {
                let left = if let TextureData::RgbU8(data) = left.data {
                    data
                } else {
                    unreachable!()
                };
                let top = if let TextureData::RgbU8(data) = top.data {
                    data
                } else {
                    unreachable!()
                };
                let bottom = if let TextureData::RgbU8(data) = bottom.data {
                    data
                } else {
                    unreachable!()
                };
                let front = if let TextureData::RgbU8(data) = front.data {
                    data
                } else {
                    unreachable!()
                };
                let back = if let TextureData::RgbU8(data) = back.data {
                    data
                } else {
                    unreachable!()
                };
                TextureCubeData::RgbU8(right, left, top, bottom, front, back)
            }
            TextureData::RgbaU8(right) => {
                let left = if let TextureData::RgbaU8(data) = left.data {
                    data
                } else {
                    unreachable!()
                };
                let top = if let TextureData::RgbaU8(data) = top.data {
                    data
                } else {
                    unreachable!()
                };
                let bottom = if let TextureData::RgbaU8(data) = bottom.data {
                    data
                } else {
                    unreachable!()
                };
                let front = if let TextureData::RgbaU8(data) = front.data {
                    data
                } else {
                    unreachable!()
                };
                let back = if let TextureData::RgbaU8(data) = back.data {
                    data
                } else {
                    unreachable!()
                };
                TextureCubeData::RgbaU8(right, left, top, bottom, front, back)
            }
            _ => unimplemented!(),
        };

        Ok(Self {
            data,
            width: right.width,
            height: right.height,
            min_filter: right.min_filter,
            mag_filter: right.mag_filter,
            mip_map_filter: right.mip_map_filter,
            wrap_s: right.wrap_s,
            wrap_t: right.wrap_t,
            wrap_r: right.wrap_s,
            ..Default::default()
        })
    }
}

impl Loaded {
    ///
    /// Deserialize the loaded image resource at the given path into a [Texture2D].
    ///
    pub fn image<P: AsRef<Path>>(&mut self, path: P) -> Result<Texture2D> {
        Texture2D::deserialize(&self.get_bytes(path)?)
    }

    ///
    /// Deserialize the 6 loaded image resources at the given paths into a [TextureCube].
    ///
    pub fn cube_image<P: AsRef<Path>>(
        &mut self,
        right_path: P,
        left_path: P,
        top_path: P,
        bottom_path: P,
        front_path: P,
        back_path: P,
    ) -> Result<TextureCube> {
        TextureCube::from_bytes(
            self.get_bytes(right_path)?,
            self.get_bytes(left_path)?,
            self.get_bytes(top_path)?,
            self.get_bytes(bottom_path)?,
            self.get_bytes(front_path)?,
            self.get_bytes(back_path)?,
        )
    }
}