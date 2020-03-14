use super::*;

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

const MEDIA_LAB: (f64, f64) = (42.3603574_f64, -71.0872641_f64);
const KENDAL_STATION: (f64, f64) = (42.362484_f64, -71.085577_f64);

#[test]
fn test_location_distance() {
    let lab = Location::new(MEDIA_LAB.0, MEDIA_LAB.1);
    let station = Location::new(KENDAL_STATION.0, KENDAL_STATION.1);
    let threshold = 280.0; // meters, got this approximate distance using google maps
    let distance = lab.distance(&station);
    assert!(distance < threshold);
}

//
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
//     let far_away = Location::new(35.6804114,139.7690105); // somewhere close to Tokyo Station
//     let fence = Fence::new(MEDIA_LAB, 300, now, 60*60);
//     let after_half_hour = now + Duration::minutes(30);
//
//     // prove
//     let proof = hide(fence, far_away, after_half_hour);
//
//     // verify
//     assert!(!proof.seek().is_ok());
// }
