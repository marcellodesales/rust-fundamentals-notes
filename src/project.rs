
pub mod ProjectV1 {

    // This declaration DEPENDS ON THE MAIN FUNCTION pub mod geo declaration!!!!
    // https://stackoverflow.com/questions/46829539/how-to-include-files-from-same-directory-in-a-module-using-cargo-rust/46829631#46829631
    // https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
    use crate::geo;

    // we can use aliases for functions if there are collisions around the function names
    //use crate::geo::Calculations::distance as distance_calc;

    // HAVERSINE FORMULA
    pub fn calculate_distance_between_airports() {
        let kcle_lat_degrees: f64 = 41.4075;
        let kcle_lon_degrees: f64 = -81.851111;

        let kslc_lat_degrees: f64 = 40.7861;
        let kslc_lon_degrees: f64 = -111.9822;

        let distance = geo::Calculations::distance(kcle_lat_degrees,
                                              kcle_lon_degrees, kslc_lat_degrees,
                                              kslc_lon_degrees);
        println!("The distance between two points is {:.1} kolometers", distance);
    }
}

pub mod ProjectV2 {

    use crate::geo;

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
                    let distance = geo::Calculations::distance(previous_value.1,
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