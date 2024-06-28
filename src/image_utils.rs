use bellman::pairing::bn256::Fr;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};

pub fn load_and_hash_image(image_path: &str) -> Result<Fr, Box<dyn std::error::Error>> {
    let img = ImageReader::open(image_path)?.decode()?;
    let (width, height) = img.dimensions();
    let mut hasher = Sha256::new();

    for pixel in img.pixels() {
        hasher.update(&pixel.2 .0);
    }

    let result = hasher.finalize();
    let hash_bytes = result.as_slice();

    let hash = Fr::from_str(&format!("{:x}", hash_bytes))?;

    Ok(hash)
}
