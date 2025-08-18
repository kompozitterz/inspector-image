use crate::domain::stego::{Stego, StegoError};
use std::path::Path;
use image::{io::Reader as ImageReader, GenericImageView};

pub struct LsbStego;

impl Stego for LsbStego {
    fn extract(path: &Path) -> Result<String, StegoError> {
        let img = ImageReader::open(path)
            .map_err(|e| StegoErro::ImageError(e.to_string()))?
            .decode()
            .map_err(|e| StegoError::ImageError(e.to_string()))?;

        let (w, h) = img.dimensions();
        let mut bits = Vec::new();

        for y in 0..h {
            for x in 0..w {
                let pixel = img.get_pixel(x, y);
                let lsb = pixel[0] & 1;
                bits.push(lsb);
            }
        }
        let bytes: Vec<u8> = bits.chunks(8)
            .map (|chunk| chunk.iter().fold(0, |acc, &b| (acc >>1) | b))
            .collect();

        let text = String::from_utf8(bytes)
            .map_err(|e| StegoError::Utf8Error(e.to_string()))?;

        if text.contains("----BEGIN PGP PUBLIC KEY BLOCK----") {
            Ok(text)
        } else {
            Err(StegoError::PgpNotFound)
        }
    }
}