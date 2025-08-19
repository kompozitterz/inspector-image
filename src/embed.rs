use image::{io::Reader as ImageReader, DynamicImage, GenericImage, GenericImageView};
use std::fs;

/// Cache un message texte (ex: clé PGP) dans une image en utilisant le LSB des 3 canaux (R, G, B).
pub fn hide_message(input: &str, output: &str, message_file: &str) {
    // Charger l’image
    let mut img = ImageReader::open(input)
        .expect("Impossible d’ouvrir l’image")
        .decode()
        .expect("Impossible de décoder l’image")
        .to_rgb8();

    // Charger le message (PGP key ou autre)
    let mut message = fs::read_to_string(message_file).expect("Impossible de lire le message");

    // Ajouter un marqueur de fin "<END>"
    message.push_str("<END>");

    // Convertir en bits
    let bits: Vec<u8> = message
        .bytes()
        .flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1))
        .collect();

    let (w, h) = img.dimensions();
    let mut idx = 0;

    'outer: for y in 0..h {
        for x in 0..w {
            let mut pixel = img.get_pixel_mut(x, y);

            // 🔥 cacher dans les 3 canaux R, G, B
            for c in 0..3 {
                if idx < bits.len() {
                    pixel[c] = (pixel[c] & !1) | bits[idx];
                    idx += 1;
                } else {
                    break 'outer;
                }
            }
        }
    }

    img.save(output).expect("Impossible de sauvegarder l’image");
    println!("✅ Message caché dans {}", output);
}

// Petit main pour lancer en CLI
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: embed <input.png> <output.png> <message.txt>");
        return;
    }
    hide_message(&args[1], &args[2], &args[3]);
}
