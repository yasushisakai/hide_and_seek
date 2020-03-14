use super::*;

#[test]
fn test_bulletproof_basic() {
    let secret: u64 = u32::max_value().into(); // max value but still in u32
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);
    let mut transcript = Transcript::new(b"single proof");
    let scalar = Scalar::random(&mut thread_rng());

    let (proof, value) =
        RangeProof::prove_single(&bp_gens, &pc_gens, &mut transcript, secret, &scalar, 32)
            .expect("could not make a proof");

    let mut transcript = Transcript::new(b"single proof"); // shadow this

    let result = proof.verify_single(&bp_gens, &pc_gens, &mut transcript, &value, 32);

    assert!(result.is_ok())
}



#[test]
fn test_bulletproof_basic_fail_1() {
    let mut secret: u64 = u32::max_value().into();
    secret += 1; // this kicks out of the u32 range
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);
    let mut transcript = Transcript::new(b"single proof");
    let scalar = Scalar::random(&mut thread_rng());

    let (proof, value) =
        RangeProof::prove_single(&bp_gens, &pc_gens, &mut transcript, secret, &scalar, 32)
            .expect("could not make a proof");
    let mut transcript = Transcript::new(b"single proof"); // shadow this
    let result = proof.verify_single(&bp_gens, &pc_gens, &mut transcript, &value, 32);
    assert!(!result.is_ok()) // this should be false because we are out of range
}

#[test]
fn test_bulletproof_basic_fail_2() {
    let mut secret: u64 = 1u32.into();
    secret = secret << 32; // shift 32 zeros goes out of 32 bit range
    println!("secret: {}",secret);
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(64, 1);
    let mut transcript = Transcript::new(b"single proof");
    let scalar = Scalar::random(&mut thread_rng());

    let (proof, value) =
        RangeProof::prove_single(&bp_gens, &pc_gens, &mut transcript, secret, &scalar, 32)
            .expect("could not make a proof");
    let mut transcript = Transcript::new(b"single proof"); // shadow this
    let result = proof.verify_single(&bp_gens, &pc_gens, &mut transcript, &value, 32);
    assert!(!result.is_ok()) // this should be false because we are out of range
}

// const MEDIA_LAB = Location::new(42.3603574, -71.0872641);

// #[test]
// fn test_hide_and_seek_basic() {
//
//     let now = Utc::now();
//     let fence = Fence::new(MEDIA_LAB, 300, now, 60*60); // {300m, one hour} range
//     let after_half_hour = now + Duration::minutes(30);
//
//     // prove
//     let proof = hide(fence, MEDIA_LAB, after_half_hour);
//
//     // verify
//     assert!(proof.seek().is_ok());
// }
//
// #[test]
// fn test_hide_and_sekk_basic_fail() {
//
//     let now = Utc::now();
//     let far_away = Location::new(35.6804114,139.7690105); // somewhere close to Tokyo
//     let fence = Fence::new(MEDIA_LAB, 300, now, 60*60);
//     let after_half_hour = now + Duration::minutes(30);
//
//     // prove
//     let proof = hide(fence, far_away, after_half_hour);
//
//     // verify
//     assert!(!proof.seek().is_ok());
// }
