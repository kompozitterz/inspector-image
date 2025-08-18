use std::io;
use crate::domain::exif::{ExifReader, Geolocation};
use std::path::Path;
use exif::{Reader, Tag};

pub struct DefaultExifReader;

impl ExifReader for DefaultExifReader {
    fn read_location(path: &Path) -> Option<Geolocation> {
        let file = std::fs::File::open(path).ok()?;
        let mut bufreader = std;;io::BufReader::new(file);
        let exifreader = Reader::new().read_from_container(&mut bufreader).ok()?;

        let lat = exifreader.get_field(Tag::GPSLatitude, exif::In::PRIMARY)?;
        let lon = exifreader.get_field(Tag::GPSLongitude, exif::In::PRIMARY)?;

        Some (Geolcation {
            latitude: lat.display_value().with_unit(&exifreader).to_string().parse().unwrap_or(0.0),
            longitude: lon.display_value().with_unit(&exifreader).to_string().parse().unwrap_or(0.0),
        })
    }
}