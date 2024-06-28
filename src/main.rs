mod circuit;
mod generate_proof;
mod image_utils;

use generate_proof::{generate_proof, verify_proof};

fn main() {
    let image_path = "image.png";

    generate_proof(image_path).expect("Proof generation failed");
    verify_proof().expect("Proof verification failed");
}
