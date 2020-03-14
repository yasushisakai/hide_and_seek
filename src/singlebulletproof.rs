use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use curve25519_dalek::{ristretto::CompressedRistretto, scalar::Scalar};
use merlin::Transcript;
use rand::thread_rng;

pub struct SingleBulletProof {
    rp: RangeProof,
    cv: CompressedRistretto
}

const TRANSCRIPT: &'static[u8;25] = b"simple_single_bulletproof";

impl SingleBulletProof {

    pub fn new(secret: u64) -> Self {
        let pc_gens = PedersenGens::default();
        let bp_gens = BulletproofGens::new(64, 1);
        let mut transcript = Transcript::new(TRANSCRIPT);
        let scalar = Scalar::random(&mut thread_rng());
        let (rp, cv) = RangeProof::prove_single(&bp_gens, &pc_gens, &mut transcript, secret, &scalar, 32).expect("could not make a proof");
        Self {rp, cv}
    }

    pub fn verify(&self) -> bool {
        let pc_gens = PedersenGens::default();
        let bp_gens = BulletproofGens::new(64,1);
        let mut transcript = Transcript::new(TRANSCRIPT);
        let result = self.rp.verify_single(&bp_gens, &pc_gens, &mut transcript, &self.cv, 32);
        result.is_ok()
    }

}
