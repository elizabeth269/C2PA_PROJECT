# C2PA with zk-SNARKs in Rust

This project demonstrates the integration of zk-SNARKs with the C2PA (Content Authenticity Initiative) standard using Rust. The example includes loading an image, hashing its contents using SHA-256, and proving and verifying the hash using zk-SNARKs.

## Dependencies

Ensure you have the following dependencies in your `Cargo.toml`:

```toml
[dependencies]
bellman = "0.10.0"
pairing = "0.22.0"
rand = "0.8.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sha2 = "0.10.6"
image = "0.24.4"
```

### Project Structure

circuit.rs: Defines the zk-SNARK circuit.
generate_proof.rs: Functions to generate and verify zk-SNARK proofs.
image_utils.rs: Utility functions to load and hash images.
main.rs: Entry point of the application.

# Usage

Building the Project

1. Clone the repository: git clone https://github.com/elizabeth269/C2PA_PROJECT.git

2. Build the project: cargo build

Running the Project

1. cargo run

# Here's a brief overview of how the project works:

The image_utils.rs file contains a function load_and_hash_image that reads an image, hashes its pixel values using SHA-256, and returns the hash.
The circuit.rs file defines a zk-SNARK circuit that verifies the hash.
The generate_proof.rs file contains functions to generate and verify zk-SNARK proofs using the circuit defined in circuit.rs.
The main.rs file ties everything together, loading the image, generating the proof, and verifying the proof.

License
This project is licensed under the MIT License.

Feel free to adjust the content to better fit your project specifics or add more details as needed.
