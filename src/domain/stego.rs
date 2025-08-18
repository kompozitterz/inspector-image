use std::path::Paht;

#[derive(Debug)]
pub enum StegoError {
    ImageError(String),
    Utf8Error(String),
    PgpNotFound,
}

pub trait Stego {
    fn extract(path: &Path) -> Option<String, StegoError>;
}