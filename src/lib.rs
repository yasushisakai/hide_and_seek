mod fence;
mod location;

use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek::{ristretto::CompressedRistretto, scalar::Scalar};
use merlin::Transcript;
use rand::thread_rng;

pub fn hide(t: f64) -> (RangeProof, CompressedRistretto) {
    // TODO: is mapping to the whole f64 space needed??
    let fbytes = (f64::MAX * t).to_be_bytes();
    let secret = u64::from_be_bytes(fbytes);

    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);
    let mut transcript = Transcript::new(b"single_proof");
    let scalar = Scalar::random(&mut thread_rng());

    RangeProof::prove_single(&bp_gens, &pc_gens, &mut transcript, secret, &scalar, 32)
        .expect("could not make a proof")
}

#[cfg(test)]
mod tests;
