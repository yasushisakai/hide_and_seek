use super::*;
use std::fs::write;

#[test]
fn test_bulletproof_basic() {
    let proof = SingleBulletProof::new(0u64);
    assert!(proof.verify());
}

fn test_bulletproof_basic_fail() {
    let max32: u64 = u32::max_value().into();

    // this should pass
    let proof = SingleBulletProof::new(max32);
    assert!(proof.verify());

    // this should fail
    let over_range = max32 + 1;
    let proof = SingleBulletProof::new(over_range);
    assert!(!proof.verify());

    // this should fail too
    let over_range_2 = 1u64 << 32;
    let proof = SingleBulletProof::new(over_range_2);
    assert!(!proof.verify());
}

const MEDIA_LAB: (f64, f64) = (42.3603574, -71.0872641);
const KENDAL_STATION: (f64, f64) = (42.362484, -71.085577);
const TOKYO_STATION: (f64, f64) = (35.6804114,139.7690105);

#[test]
fn test_location_distance() {
    let lab = Location::from_tuple(&MEDIA_LAB);
    let station = Location::from_tuple(&KENDAL_STATION);
    let threshold = 280.0; // meters, got this approximate distance using google maps
    let distance = lab.distance(&station);
    assert!(distance < threshold);
}


#[test]
fn test_hide_and_seek_basic() {
   
    let now = Utc::now();
    let lab = Location::from_tuple(&MEDIA_LAB);
    let station = Location::from_tuple(&KENDAL_STATION);
    let fence = Fence::new(lab, 300.0, now.naive_utc(), 60*60); // {300m, one hour} range
    let after_half_hour = now + Duration::minutes(30);

    // prove
    let proof = fence.hide(&station, after_half_hour.naive_utc());

    // verify
    assert!(proof.verify());
}

#[test]
fn test_hide_and_seek_basic_fail_too_far() {

    let now = Utc::now();
    let lab = Location::from_tuple(&MEDIA_LAB);
    let tokyo = Location::from_tuple(&TOKYO_STATION);
    let fence = Fence::new(lab, 300.0, now.naive_utc(), 60*60);
    let after_half_hour = now + Duration::minutes(30);

    // prove
    let proof = fence.hide(&tokyo, after_half_hour.naive_utc());

    // verify
    assert!(!proof.verify());
}

#[test]
fn test_hide_and_seek_fail_wrong_time() {

    let now = Utc::now();
    let lab = Location::from_tuple(&MEDIA_LAB);
    let station = Location::from_tuple(&KENDAL_STATION);
    let fence = Fence::new(lab, 300.0, now.naive_utc(), 60*60); // {300m, one hour} range
    let after_half_hour = now + Duration::minutes(90);

    // prove
    let proof = fence.hide(&station, after_half_hour.naive_utc());

    // verify
    assert!(!proof.verify());
}

#[test]
fn test_verify_serialize(){

    let now = Utc::now();
    let lab = Location::from_tuple(&MEDIA_LAB);
    let station = Location::from_tuple(&KENDAL_STATION);
    let fence = Fence::new(lab, 300.0, now.naive_utc(), 60*60);
    let half_past = now + Duration::minutes(30);
    let proof = fence.hide(&station, half_past.naive_utc());

    let mut file = std::fs::File::create("test.proof").unwrap();
    std::fs::write("test.proof", proof.to_bytes());

    let mut file = std::fs::File::open("test.proof").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let new_proof = SingleBulletProof::from_bytes(&buffer);

    assert!(new_proof.verify());
}
