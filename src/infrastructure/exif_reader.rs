use crate::domain::exif::{ExifReader, Geolocation};
use std::path::Path;
use rexiv2::Metadata;

pub struct DefaultExifReader;

impl ExifReader for DefaultExifReader {
    fn read_location(path: &Path) -> Option<Geolocation> {
        // Charger les métadonnées EXIF
        let meta = Metadata::new_from_path(path).ok()?;

        // Ici get_tag_string retourne Result<String, Error>
        let lat_str = meta.get_tag_string("Exif.GPSInfo.GPSLatitude").ok()?;
        let lon_str = meta.get_tag_string("Exif.GPSInfo.GPSLongitude").ok()?;
        let lat_ref = meta.get_tag_string("Exif.GPSInfo.GPSLatitudeRef").ok()?;
        let lon_ref = meta.get_tag_string("Exif.GPSInfo.GPSLongitudeRef").ok()?;

        // Convertir les coordonnées EXIF en f64
        let mut lat = parse_gps(&lat_str)?;
        let mut lon = parse_gps(&lon_str)?;

        // Appliquer le signe en fonction de la référence N/S/E/W
        if lat_ref == "S" {
            lat = -lat;
        }
        if lon_ref == "W" {
            lon = -lon;
        }

        Some(Geolocation { latitude: lat, longitude: lon })
    }
}

/// Convertit une valeur EXIF GPS comme "48/1 51/1 0/1" en degrés décimaux (f64)
fn parse_gps(coord: &str) -> Option<f64> {
    let parts: Vec<&str> = coord.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }

    let deg = parse_fraction(parts[0])?;
    let min = parse_fraction(parts[1])?;
    let sec = parse_fraction(parts[2])?;

    Some(deg + (min / 60.0) + (sec / 3600.0))
}

/// Convertit un rationnel EXIF "48/1" → f64
fn parse_fraction(s: &str) -> Option<f64> {
    let nums: Vec<&str> = s.split('/').collect();
    if nums.len() == 2 {
        let num: f64 = nums[0].parse().ok()?;
        let den: f64 = nums[1].parse().ok()?;
        Some(num / den)
    } else {
        None
    }
}
