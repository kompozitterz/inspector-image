use crate::domain::stego::{Stego, StegoError};
use std::path::Path;

pub fn extract_gps<S: Stego>(stego: S, path: &Path) -> Result<String, StegoError> {
    S::extract(path)
}