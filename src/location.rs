use std::ops;

pub type Meters = f64;

pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

impl Location {

    pub fn new(lat: f64, lng:f64) -> Self{
        Self{lat, lng}
    }

    fn to_radians(&self) -> (f64, f64) {
        (self.lat.to_radians(), self.lng.to_radians())
    }

    pub fn distance(&self, other: &Location) -> Meters {
        let earth_radius = 6371.0_f64 * 1000_f64;
        let arlat = self.lat.to_radians();
        let brlat = self.lat.to_radians();
        let (dlat, dlng) = (self - other).to_radians();
        let central_angle_inner =
            (dlat / 2.0).sin().powi(2) + arlat.cos() * brlat.cos() * (dlng / 2.0).sin().powi(2);
        let central_angle = 2.0 * central_angle_inner.sqrt().asin();
        earth_radius * central_angle // in meters
    }
}

impl<'a, 'b> ops::Sub<&'b Location> for &'a Location {
    type Output = Location;

    fn sub(self, rhs: &'b Location) -> Location {
        Location {
            lat: self.lat - rhs.lat,
            lng: self.lng - rhs.lng,
        }
    }
}
