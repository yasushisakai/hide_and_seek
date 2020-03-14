use crate::location::{Meters, Location};
use blake3::hash;
use chrono::prelude::*;
use chrono::Duration;

pub struct Fence {
    loc: Location,
    radius: Meters,
    start: NaiveDateTime,
    delta: Duration,
}

impl Fence {

    pub fn new(lat: f64, lng: f64, radius: Meters, start: NaiveDateTime, delta_sec:u32) -> Self{
        let delta = Duration::seconds(delta_sec.into());
        let loc = Location::new(lat, lng);
        Self{
            loc, radius, start, delta
        }
    }

    fn hash(&self) -> [u8; 32] {
        let mut raw_bytes: [u8; 40] = [0u8; 40];
        raw_bytes.clone_from_slice(&self.loc.lat.to_be_bytes());
        raw_bytes[8..].clone_from_slice(&self.loc.lng.to_be_bytes());
        raw_bytes[16..].clone_from_slice(&self.radius.to_be_bytes());
        raw_bytes[24..].clone_from_slice(&self.start.timestamp().to_be_bytes());
        raw_bytes[32..].clone_from_slice(&self.delta.num_seconds().to_be_bytes());
        println!("{:?}", String::from_utf8(raw_bytes.to_vec()));
        hash(&raw_bytes).into()
    }

    fn is_inside(&self, test_point: &Location, time: &NaiveDateTime) -> bool {
        self.loc.distance(test_point) < self.radius
            && self.start < *time
            && time < &(self.start + self.delta)
    }
}
