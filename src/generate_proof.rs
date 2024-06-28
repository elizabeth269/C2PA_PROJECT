use bellman::groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
};
use bellman::pairing::bn256::{Bn256, Fr};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};

use crate::circuit::ProvenanceCircuit;
use crate::image_utils::load_and_hash_image;

#[derive(Serialize, Deserialize)]
struct ProofData {
    proof: Vec<u8>,
    public_inputs: Vec<Fr>,
}

pub fn generate_proof(image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let hash = load_and_hash_image(image_path)?;

    let params = {
        let rng = &mut thread_rng();
        generate_random_parameters::<Bn256, _, _>(ProvenanceCircuit { hash: None }, rng)?
    };

    let pvk = prepare_verifying_key(&params.vk);

    let proof = {
        let rng = &mut thread_rng();
        create_random_proof(ProvenanceCircuit { hash: Some(hash) }, &params, rng)?
    };

    let mut proof_vec = vec![];
    proof.write(&mut proof_vec)?;

    let proof_data = ProofData {
        proof: proof_vec,
        public_inputs: vec![hash],
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
        generate_random_parameters::<Bn256, _, _>(ProvenanceCircuit { hash: None }, rng)?
    };

    let pvk = prepare_verifying_key(&params.vk);

    let proof = {
        let mut proof_vec = &proof_data.proof[..];
        bellman::groth16::Proof::read(&mut proof_vec)?
    };

    let result = verify_proof(&pvk, &proof, &proof_data.public_inputs)?;
    if result {
        println!("Proof is valid");
    } else {
        println!("Proof is invalid");
    }

    Ok(())
}
