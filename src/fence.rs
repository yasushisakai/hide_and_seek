use crate::location::{Meters, Location};
use crate::singlebulletproof::SingleBulletProof;
use blake3::hash;
use chrono::prelude::*;
use chrono::Duration;
use serde::{Serialize, Deserialize};

type Seconds = i64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fence {
    center: Location,
    radius: Meters,
    start: NaiveDateTime,
    delta: Seconds,
}

impl Fence {

    pub fn new(center: Location, radius: Meters, start: NaiveDateTime, delta:Seconds) -> Self{
        Self{
            center, radius, start, delta
        }
    }

    pub fn duration(&self) -> Duration {
        Duration::seconds(self.delta)
    }

    fn hash(&self) -> [u8; 32] {
        let mut raw_bytes: [u8; 40] = [0u8; 40];
        raw_bytes.clone_from_slice(&self.center.lat.to_be_bytes());
        raw_bytes[8..].clone_from_slice(&self.center.lng.to_be_bytes());
        raw_bytes[16..].clone_from_slice(&self.radius.to_be_bytes());
        raw_bytes[24..].clone_from_slice(&self.start.timestamp().to_be_bytes());
        raw_bytes[32..].clone_from_slice(&self.delta.to_be_bytes());
        hash(&raw_bytes).into()
    }

    fn is_inside(&self, test_point: &Location, time: &NaiveDateTime) -> bool {
        self.center.distance(test_point) < self.radius
            && self.start < *time
            && time < &(self.start + Duration::seconds(self.delta))
    }

    pub fn hide(&self, location: &Location, time: NaiveDateTime) -> SingleBulletProof {

        // TODO: consider having the bulletproof take two commit values: location and time
        // we are currently not fully taking acount on the time range.
        // This will be trade off of proof size and computation size


        let distance = self.center.distance(location);
        let mut n_dist = distance / self.radius;

        let mut inside = self.is_in_time_range(&time);
        if n_dist < 0.0 || n_dist > 1.0 {
            inside = false;
            // we don't care too much if it was big or small
            n_dist = n_dist.abs().fract();
        }

        let mut secret:u64 = parameter_to_bytes(n_dist).into();
        if !inside {
            secret = secret << 32; // kick it out from the range.
        }

        SingleBulletProof::new(secret)
    }

    pub fn is_in_time_range(&self, time: &NaiveDateTime) -> bool {
        time > &self.start && time < &(self.start + Duration::seconds(self.delta))
    }
}


//
// we want to map a double float number ranging 0.0 < p < 1.0
// to a 2^32 range. This method is similar to floating point conversion.
// A quick test shows the following characteristics
//
// minimum resolution: numbers smaller than 0.00000000025 saturates to 0.
// maximum resolution: numbers larger than 0.999999999768 saturates to u32::max_value()
//
fn parameter_to_bytes(p: f64) -> u32 {
    let mut float = p;
    let mut number: u32 = 0;

    for i in 0..32 {
        let mut next = float * 2.0;
        if next >= 1.0 {
            number |= 1 << (31 - i); // msb first
            next -= 1.0;
        }

        if next == 0.0 {
            break;
        }

        float = next;
    }

    number
}

//
// converts back to the parameter 0 < n < 1
// note that this does not gaurantee we will get the same number
// the error rate will be +- 0.5^32 = 2.3e-10.
//
fn bytes_to_parameter(number: u32) -> f64 {

    let mut p = 0_f64;
    let mut inc = 1.0_f64;

    for i in 0..32 {
        inc *= 0.5;
        let masked = number & (1 << (31-i));
        if masked != 0 {
            p += inc;
        }
    }

    p
}
