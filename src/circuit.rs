use bellman::gadgets::boolean::Boolean;
use bellman::gadgets::num::AllocatedNum;
use bellman::gadgets::sha256::sha256;
use bellman::pairing::bn256::{Bn256, Fr};
use bellman::{Circuit, ConstraintSystem, SynthesisError};

#[derive(Clone)]
pub struct ProvenanceCircuit {
    pub author: Option<Fr>,
    pub timestamp: Option<Fr>,
}

impl Circuit<Bn256> for ProvenanceCircuit {
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let author = AllocatedNum::alloc(cs.namespace(|| "author"), || {
            self.author.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let timestamp = AllocatedNum::alloc(cs.namespace(|| "timestamp"), || {
            self.timestamp.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Convert author and timestamp to bits
        let author_bits = author.into_bits_le(cs.namespace(|| "author_bits"))?;
        let timestamp_bits = timestamp.into_bits_le(cs.namespace(|| "timestamp_bits"))?;

        // Concatenate author and timestamp bits
        let mut hash_input = vec![];
        hash_input.extend(author_bits);
        hash_input.extend(timestamp_bits);

        // Perform SHA-256 hashing
        let hash_bits = sha256(cs.namespace(|| "hash"), &hash_input)?;

        // Output the hash
        AllocatedNum::pack_bits(cs.namespace(|| "hash_num"), &hash_bits)?;

        Ok(())
    }
}
