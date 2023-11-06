fn main() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let route = [
        ("KCLE", 41.4075, -81.851111),
        ("LEYIR", 41.51030, -83.88080),
        ("PIONS", 41.65390, -84.48190),
        ("ZOSER", 41.72390, -84.78130),
        ("MODEM", 41.72800, -84.89730),
        ("BRYTO", 41.74170, -85.31320),
        ("SEWTO", 41.74780, -85.51130),
        ("GIJ", 41.76860, -86.31850),
        ("NEPTS", 41.96750, -87.05300),
        ("THORR", 42.12330, -87.60030),
        ("OBK", 42.22140, -87.95160),
        ("COTON", 42.31990, -89.31220),
        ("DBQ", 42.40150, -90.70910),
        ("VIGGR", 42.55520, -93.12410),
        ("FOD", 42.61110, -94.29480),
        ("ONL", 42.47050, -98.68690),
        ("BFF", 41.89420, -103.48200),
        ("OCS", 41.59020, -109.01500),
        ("PUDVY", 41.54270, -109.34200),
        ("WEGEM", 41.44560, -109.99000),
        ("KSLC", 40.7861, -111.9822)
    ];

    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<(&str, f64, f64)> = None;

    for waypoint in route.iter() {
        
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from(waypoint.clone());
                continue;
            }
            Some(previous_waypoint_value) => {
                let previous_waypoint_radians = previous_waypoint_value.1.to_radians();
                let waypoint_radians = waypoint.1.to_radians();

                let delta_latitude = (previous_waypoint_value.1 - waypoint.1).to_radians();
                let delta_longitude = (previous_waypoint_value.2 - waypoint.2).to_radians();

                let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(),2)
                + previous_waypoint_radians.cos() * waypoint_radians.cos()
                * f64::powi((delta_longitude / 2.0).sin(),2);
                let central_angle = 2.0 * inner_central_angle.sqrt().asin();
                let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
                total_distance += distance;
                previous_waypoint = Option::from(waypoint.clone());

                println!("The distance between {} and {} is {:.1} kilometers",
                previous_waypoint_value.0, waypoint.0, distance);
            }
        }
    }
    
    println!("\nThe total distance between the two points is {:.1} kilometers", total_distance);

}
