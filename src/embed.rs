use image::open;
use std::{env, fs};

/// Cache un message texte (ex: clé PGP) dans une image en utilisant le LSB.
pub fn hide_message(input: &str, output: &str, message_file: &str) {
    // Charger l’image
    let mut img = open(input).expect("Impossible d’ouvrir l’image").to_rgb8();

    // Charger le message (PGP key)
    let mut message = fs::read_to_string(message_file)
        .expect("Impossible de lire le fichier message");

    // Ajouter un marqueur de fin
    message.push_str("<END>");

    // Convertir en bits
    let bits: Vec<u8> = message
        .bytes()
        .flat_map(|b| (0..8).rev().map(move |i| (b >> i) & 1))
        .collect();

    let (w, h) = img.dimensions();
    let capacity = (w * h * 3) as usize;

    if bits.len() > capacity {
        panic!(
            "Message trop long : besoin de {} bits, capacité max {} bits",
            bits.len(),
            capacity
        );
    }

    let mut idx = 0;

    for y in 0..h {
        for x in 0..w {
            let pixel = img.get_pixel_mut(x, y);
            for c in 0..3 {
                if idx < bits.len() {
                    pixel[c] = (pixel[c] & !1) | bits[idx];
                    idx += 1;
                }
            }
        }
    }

    img.save(output).expect("Impossible d’enregistrer l’image");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "Usage: {} <input.png> <output.png> <message_file>",
            args[0]
        );
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];
    let message_file = &args[3];

    hide_message(input, output, message_file);
    println!("✅ Message caché dans {}", output);
}
