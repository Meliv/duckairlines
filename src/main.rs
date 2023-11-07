#![allow(unused)]

struct Waypoint {
    name: String,
    latitude: f32,
    longitude: f32,
}

struct Segment {
    start: Waypoint,
    end: Waypoint,
}

impl Segment {
    fn new(start: Waypoint, end: Waypoint) -> Self {
        Self { start, end }
    }

    fn distance(&self) -> f32 {
        const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;
        let start_radians = self.start.latitude.to_radians();
        let end_radians = self.end.latitude.to_radians();
        let delta_latitude = (self.start.latitude - self.end.latitude).to_radians();
        let delta_longitude = (self.start.longitude - self.end.longitude).to_radians();
        let inner_central_angle = f32::powi((delta_latitude / 2.0).sin(), 2)
            + start_radians.cos() * end_radians.cos() * f32::powi((delta_longitude / 2.0).sin(), 2);
        let central_angle = 2.0 * inner_central_angle.sqrt().asin();
        let distance = EARTH_RADIUS_IN_KILOMETERS as f32 * central_angle;
        distance
    }
}

fn main() {
    let kcle = Waypoint {
        name: "KCLE".to_string(),
        latitude: 41.4075,
        longitude: -81.851111,
    };

    let kslc = Waypoint {
        name: "KSLC".to_string(),
        latitude: 40.7861,
        longitude: -111.9822,
    };

    let kcle_kslc = Segment::new(kcle, kslc);

    let distance = kcle_kslc.distance();
    println!("{:.1}", distance);
}
