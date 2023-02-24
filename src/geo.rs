
pub mod Calculations {
    // https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
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