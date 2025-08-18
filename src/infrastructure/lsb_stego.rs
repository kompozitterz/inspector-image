use crate::domain::stego::{Stego, StegoError};
use std::path::Path;
use image::{io::Reader as ImageReader, GenericImageView};

pub struct LsbStego;

impl Stego for LsbStego {
    fn extract(&self, path: &Path) -> Result<String, StegoError> {
        let img = ImageReader::open(path)?.decode()?;

        let (w, h) = img.dimensions();
        let mut bits = Vec::new();
        let mut message = Vec::new();

        for y in 0..h {
            for x in 0..w {
                let pixel = img.get_pixel(x, y);
                let lsb = pixel[0] & 1;
                bits.push(lsb);

                // Quand on a 8 bits → on fabrique un byte
                if bits.len() == 8 {
                    let byte = bits.iter().fold(0, |acc, &b| (acc << 1) | b);
                    message.push(byte);
                    bits.clear();

                    // Vérifier si on a atteint le marqueur "<END>"
                    if message.ends_with(b"<END>") {
                        let text = String::from_utf8_lossy(&message[..message.len() - 5]).to_string();
                        if text.contains("-----BEGIN PGP PUBLIC KEY BLOCK-----") {
                            return Ok(text);
                        } else {
                            return Err(StegoError::PgpNotFound);
                        }
                    }
                }
            }
        }

        Err(StegoError::PgpNotFound)
    }
}
