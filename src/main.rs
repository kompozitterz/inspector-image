mod applications;
mod cli;
mod domain;
mod infrastructure;
mod embed;

use crate::cli::commands::{Cli, Commands};
use crate::applications::{map_exif, steg_extract};
use crate::infrastructure::{exif_reader::DefaultExifReader, lsb_stego::LsbStego};

use clap::Parser;
use std::path::Path;

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Map { image } => {
            let path = Path::new(&image);
            match map_exif::map_image_location(DefaultExifReader, path) {
                Some(geo) => println!("Lat/Lon:\t({}) / ({})", geo.latitude, geo.longitude),
                None => eprintln!("Impossible de lire les coordonnées GPS."),
            }
        }
        Commands::Steg { image } => {
            let path = Path::new(&image);
            match steg_extract::extract_pgp(LsbStego, path) {
                Ok(text) => println!("{}", text),
                Err(e) => eprintln!("Erreur: {:?}", e),
            }
        }
    }
}
