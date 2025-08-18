use std::io;
use std::string::FromUtf8Error;
use thiserror::Error;
use image::ImageError;

#[derive(Debug, Error)]
pub enum StegoError {
    #[error("Erreur I/O: {0}")]
    IoError(#[from] io::Error), // <-- pour ImageReader::open()

    #[error("Erreur image: {0}")]
    ImageError(#[from] ImageError), // <-- pour decode()

    #[error("Erreur UTF-8: {0}")]
    Utf8Error(#[from] FromUtf8Error), // <-- pour String::from_utf8()

    #[error("Bloc PGP non trouvé")]
    PgpNotFound,
}

pub trait Stego {
    fn extract(&self, path: &std::path::Path) -> Result<String, StegoError>;
}
