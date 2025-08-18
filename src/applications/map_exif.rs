use crate::domain::exif::{ExifReader, Geolocation};
use std::path::Path;

pub fn map_image_location<R: ExifReader>(_reader: R, path: &Path) -> Option<Geolocation> {
    R::read_location(path)
}