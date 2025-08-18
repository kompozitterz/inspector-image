use std::path::Paht;

#[derive(Debug)]
pub struct Geolocation {
    pub latitude: f64,
    pub longitude: f64,
}

pub trait ExifReader {
    fn read_location(path: &Path) -> Option<Geolocatiton>;
}