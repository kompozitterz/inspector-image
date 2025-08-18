use std::path::Path;

/// Représente une position GPS extraite des métadonnées
#[derive(Debug)]
pub struct Geolocation {
    pub latitude: f64,
    pub longitude: f64,
}

/// Interface pour tout lecteur de métadonnées Exif
pub trait ExifReader {
    fn read_location(path: &Path) -> Option<Geolocation>;
}
