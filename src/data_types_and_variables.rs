
/**
 * Public module to show all the basic data types and variables declaration!
 * https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
 */
pub mod DataTypesAndModules {

    pub fn hello_world() {
        // ! is a macro and not a function, set of code that has a name
        let unused_variable: u32 = 0;
        println!("Hello, world!");
    }

    pub fn arrays() {
        // https://doc.rust-lang.org/book/ch03-02-data-types.html

        // array with default values
        let location: [f32; 2] = [41.4094069, -81.8546911];

        // initialization
        let loc: [f32; 2] = [0.0, 0.0];

        // initialization of larger sized array with ";"
        let loc: [f64; 100] = [0.0; 100];
        println!("Initial value: {}", loc[33]);

        //print!("Location name: {}", loc[110]);
        //    |
        // 52 |     print!("Location name: {}", loc[110]);
        //    |                                 ^^^^^^^^ index out of bounds: the length is 100 but the index is 110
    }

    pub fn tuplues() {
        // We use tuples to associate other types, using ( )
        let loc: (&str, f64, f64) = ("KCLE", 41.4094069, -81.8546911);

        // tuple indexing is with var.x
        println!("Location using index name: {}, lat: {}, lon: {}", loc.0, loc.1, loc.2);

        // destruction of array or tuple, it's better for usability
        let (name, latitude, longitude) = loc;
        println!("Location using destruction name: {}, lat: {}, lon: {}", name, latitude, longitude);
    }

}
