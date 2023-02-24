
pub mod StructsAndImpls {

    /**
     * The waypoint of the code
     */
    struct Waypoint {
        name: String,
        lat: f64,
        lon: f64
    }

    struct Segment {
        start: Waypoint,
        end: Waypoint
    }


    /**
     * Initialization of structs can be used with params
     * ATTENTION: THIS IS NOT THE BEST PATTERN! SEE TRAIT BELOW!!! with new()
     */
    fn waypoint_factory(name: String, lat: f64, lon: f64) -> Waypoint {
        println!("Creating the Waypoint name={} at {},{}", name, lat, lon);
        // TODO: can use impl associated data structures initialization
        Waypoint {
            name,
            lat,
            lon,
        }
    }

    impl Segment {
        fn new(start: Waypoint, end: Waypoint) -> Self {
            Self {
                start,
                end
            } // no need to use semi-colon as it is the return
        }

        fn distance(&self) -> f64 {
            const EARTH_RADIOS_IN_KILOMETERS: f64 = 6371.0;

            let start_radians = self.start.lat.to_radians();
            let end_radians = self.end.lat.to_radians();

            let delta_lat = (self.start.lat - self.end.lat).to_radians();
            let delta_lon = (self.start.lon - self.end.lon).to_radians();

            let inner_central_angle = f64::powi((delta_lat / 2.0).sin(), 2)
                + start_radians.cos()
                * end_radians.cos()
                * f64::powi((delta_lon / 2.0).sin(), 2);

            let central_angle = 2.0 * inner_central_angle.sqrt().asin();
            let distance = EARTH_RADIOS_IN_KILOMETERS * central_angle;
            distance as f64
        }
    }

    pub fn waypoint_data_structures_with_associated_data() {
        let kcle = waypoint_factory("KLCE".to_string(), 41.4075, -81.85222);
        // let kslc = Waypoint{
        //     // Rust will use the provided initialization and fill in the others for you
        //     name: "KSLC".to_string(),
        //     ..kcle
        // };
        let kslc = Waypoint{
            name: "KSLC".to_string(),
            lat: 40.7861,
            lon: -111.9832
        };
        // new is a trait added to the segment
        let kcle_kslc = Segment::new(kcle, kslc);
        let distance = kcle_kslc.distance();
        println!("Distance from kcle to kslc is '{:.1}' kilometers", distance)
    }
}

pub mod Traits {

    use crate::geo;

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

        println!("Is boeing flight legal? {}\nIs airbus flight legal: {}", b_l, a_l);

        println!("Now generating a random value: {}", geo::Calculations::make_random_number());
    }
}