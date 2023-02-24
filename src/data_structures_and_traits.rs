
pub mod StructsAndTraits {

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