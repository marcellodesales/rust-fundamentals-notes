

mod geo {
    const EARTH_RADIOS_IN_KILOMETERS: f64 = 6371.0;

    // private method/function, as everything is by default private in modules!
    // this is the encapsulation of methods.
    pub fn distance(start_lat: f64, start_lon: f64, end_lat: f64, end_lon: f64) -> f64 {
        let delta_lat = (start_lat - end_lat).to_radians();
        let delta_lon = (start_lon - end_lon).to_radians();

        // https://doc.rust-lang.org/std/

        let inner_central_angle = f64::powi((delta_lat / 2.0).sin(), 2)
            + start_lat.to_radians().cos()
            * end_lat.to_radians().cos()
            * f64::powi((delta_lon / 2.0).sin(), 2);

        let central_angle = 2.0 * inner_central_angle.sqrt().asin();
        EARTH_RADIOS_IN_KILOMETERS * central_angle
    }
}

pub mod ProjectV1 {

    use crate::project::geo;
    use crate::project;

    // HAVERSINE FORMULA
    pub fn calculate_distance_between_airports() {
        let kcle_lat_degrees: f64 = 41.4075;
        let kcle_lon_degrees: f64 = -81.851111;

        let kslc_lat_degrees: f64 = 40.7861;
        let kslc_lon_degrees: f64 = -111.9822;

        let distance = project::geo::distance(kcle_lat_degrees,
                                              kcle_lon_degrees, kslc_lat_degrees,
                                              kslc_lon_degrees);
        println!("The distance between two points is {:.1} kolometers", distance);
    }
}

pub mod ProjectV2 {

    use crate::project::geo;

    pub fn calculate_distance_between_airports() {
        // array of routes
        let route = [
            ("KCLE", 41.4075, -81.851111),
            ("LEYIR", 41.51030, -83.88080),
            ("PIONS", 41.65390, -84.48190),
            ("ZOSER", 41.72390, -84.78130),
            ("MODEM", 41.72800, -84.89730),
            ("BRYTO", 41.74170, -85.31320),
            ("SEWTO", 41.74780, -85.51130),
            ("GIJ", 41.76860, -81.31850),
            ("NEPTS", 41.96750, -87.05300),
            ("THORR", 42.12330, -87.60030),
            ("OBK", 42.22140, -87.95160),
            ("COTON", 42.31990, -89.31220),
            ("DBQ", 41.40150, -89.31220),
            ("VIGGR", 42.555220, -93.12410),
            ("FOD", 42.61110, -94.29480),
            ("ONL", 42.47050, -98.68690),
            ("BFF", 42.89420, -103.48200),
            ("OCS", 41.59020, -109.01500),
            ("PUDVY", 41.54270, -109.34200),
            ("WEGEM", 41.44560, -109.99000),
            ("KSLC", 40.7861, -111.9822)
        ];

        let mut total_distance :f64 = 0.0;

        // Options is the way to implement generics
        // Options can declare data types
        let mut previous_waypoint: Option<(&str, f64, f64)> = None;

        for waypoint in route.iter() {
            match previous_waypoint {
                None => {
                    previous_waypoint = Option::from(waypoint.clone());
                    continue;
                }
                Some(previous_value) => {
                    let distance = geo::distance(previous_value.1,
                                                 previous_value.2, waypoint.1,
                                                 waypoint.2);
                    total_distance += distance;

                    // set the current as previous
                    previous_waypoint = Option::from(waypoint.clone());
                    println!("The distance between '{}' and '{}' is '{:.1}' kolometers",
                             previous_value.0, waypoint.0, distance);
                }
            }
        }
    }
}

pub mod ProjectV3 {

    struct Boeing {
        required_crew: u8,
        range: u16
    }

    struct Airbus {
        required_crew: u8,
        range: u16
    }

    // Traits is
    trait Flight {
        fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool;
    }

    impl Flight for Boeing {
        fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
            // boeing you must have enough fuel for the destination + 150 miles
            available_crew >= required_crew && range + 150 > distance
        }
    }

    impl Flight for Airbus {
        fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
            // boeing you must have enough fuel for the destination + 280 miles
            available_crew >= required_crew && range + 280 > distance
        }
    }

    pub fn traits_for_structs_and_impl() {
        let boeing = Boeing{
            required_crew: 4,
            range: 7403
        };
        let airbus = Airbus{
            required_crew: 7,
            range: 5280
        };

        let b_l = boeing.is_legal(boeing.required_crew, 18,
                                  boeing.range, 2385);
        let a_l = airbus.is_legal(airbus.required_crew, 3,
                                  airbus.range, 2200);

        println!("Is boeing flight legal? {}\nIs airbus flight legal: {}", b_l, a_l)
    }
}