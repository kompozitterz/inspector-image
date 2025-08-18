use thiserror::Error;

#[derive(Debug, Error)]
pub enum StegoError {
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("PGP block not found")]
    PgpNotFound,
}

pub trait Stego {
    fn extract(&self, path: &std::path::Path) -> Result<String, StegoError>;
}
