use bellman::groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
};
use bellman::pairing::bn256::{Bn256, Fr};
use pairing::Engine;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

use crate::circuit::ProvenanceCircuit;
use crate::image_utils::load_and_hash_image;

#[derive(Serialize, Deserialize)]
struct ProofData {
    proof: Vec<u8>,
    public_inputs: Vec<String>, // Store as strings
}

pub fn generate_proof(image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let hash = load_and_hash_image(image_path)?;

    let params = {
        let rng = &mut thread_rng();
        generate_random_parameters::<Bn256, _, _>(ProvenanceCircuit::new(None), rng)?
    };

    let pvk = prepare_verifying_key(&params.vk);

    let proof = {
        let rng = &mut thread_rng();
        create_random_proof(ProvenanceCircuit::new(Some(hash)), &params, rng)?
    };

    let mut proof_vec = vec![];
    proof.write(&mut proof_vec)?;

    let proof_data = ProofData {
        proof: proof_vec,
        public_inputs: vec![hash.to_string()], // Convert Fr to String
    };

    let file = File::create("proof.json")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &proof_data)?;

    Ok(())
}

pub fn verify_proof() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("proof.json")?;
    let reader = BufReader::new(file);
    let proof_data: ProofData = serde_json::from_reader(reader)?;

    let params = {
        let rng = &mut thread_rng();
        generate_random_parameters::<Bn256, _, _>(ProvenanceCircuit::new(None), rng)?
    };

    let pvk = prepare_verifying_key(&params.vk);

    let proof = bellman::groth16::Proof::read(&proof_data.proof[..])?;

    // Convert String back to Fr
    let public_inputs: Vec<Fr> = proof_data
        .public_inputs
        .iter()
        .map(|s| Fr::from_str(s).expect("Failed to parse Fr"))
        .collect();

    let result = verify_proof(&pvk, &proof, &public_inputs)?;
    if result {
        println!("Proof is valid");
    } else {
        println!("Proof is invalid");
    }

    Ok(())
}
