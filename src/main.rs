// for unused variables
#![allow(unused_variables)]

// entrypoint of the command, () as regular function

// To view macros, install the subcommand "cargo install cargo-expand"
// Go to the terimnal and type

/*

1. Install cargo subcommand https://github.com/dtolnay/cargo-expand
2. Set the nightly build to avoid the error
   https://stackoverflow.com/questions/48675235/error-the-option-z-is-only-accepted-on-the-nightly-compiler/48675452#48675452
3. Re-run the command expand

$ cargo expand --bin rust-fundamentals --color=always --tests
Compiling rust-fundamentals v0.1.0 (/Users/marcellodesales/dev/github.com/marcellodesales/rust-fundamentals)
Finished test [unoptimized + debuginfo] target(s) in 0.30s

#![feature(prelude_import)]
#![allow(unused_variables)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[allow(dead_code)]
fn main() {
{
    ::std::io::_print(format_args!("Hello, world!\n"));
};
}
#[rustc_main]
pub fn main() -> () {
extern crate test;
test::test_main_static(&[])
}

 */

fn data_types_and_variables() {
    // ! is a macro and not a function, set of code that has a name
    let unused_variable: u32 = 0;
    println!("Hello, world!");

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

    // We use tuples to associate other types, using ( )
    let loc: (&str, f64, f64) = ("KCLE", 41.4094069, -81.8546911);

    // tuple indexing is with var.x
    println!("Location using index name: {}, lat: {}, lon: {}", loc.0, loc.1, loc.2);

    // destruction of array or tuple, it's better for usability
    let (name, latitude, longitude) = loc;
    println!("Location using destruction name: {}, lat: {}, lon: {}", name, latitude, longitude);
}

fn strings_and_string_slices() {
    // Strings mutable, stored in the heap because it can grow and shrink in size
    // The size is not constant so it cannot be stored in the stack
    let person_name_string: String = "Donald Mallard".to_string();
    let person_name_string: String = String::from("Donald Mallard");

    // String Slices are Immutable, can be stored in heap, stack or embedded in the compiled code
    let person_name_slice: &str = "Donald Mallard";

    // Just using the & in front of the variable to get its pointer memory address where data lives
    // This is the so-called de-referencing
    let person_slice_2: &String = &person_name_string;

    // Converting String to &str is simple, we know the string is in the heap
    let person_sliced: &str = person_name_string.as_str();

    // Concatenation with Strings in rust is complicated
    // Slices are immutable! so we can concatenate
    let duck = "Duck";
    let airlines = "Airlines";

    let airline_name = [duck, " ", airlines].concat();
    println!("Airline name using array concat: {}", airline_name);

    let airline_name = format!("{} {}", duck, airlines);
    println!("Airline name using format: {}", airline_name);
    // can't change the value of immutable strings
    // airline_name = airline_name + "d";

    // mutable strings so it can change
    let mut slogan = String::new();
    // push a string
    slogan.push_str("We hit the gorund");
    // push just a character with single quotes
    slogan.push(' ');
    // concatenate
    slogan = slogan + "every time";
    println!("Our slogan: {}", slogan);
}

fn numbers_variables() {
    // Rust can infer the data type at compile type, so there's no need to declare the type
    //let name:type = initial value;

    // if we don't declare the type, rust will get the default values
    // For integer, it will be the signed integer 32 bits
    let a = 32;
    // for Float point, it will be f64, as it will be shown in the IDEA
    let b = 3.4;
    let bb: f32 = 4.5;
    // In order to change the type, just declare it
    let c: i16 = 5;
    // the other way is to declare the number_type
    let d = 45_i8;

    // to avoid warning in the variables
    let _warning_variable = 0;

    // Casting variables
    let float_third_two: f32 = 17.2;
    let unsigned_eight: u8 = 5;
    // the compiler can't do implicit casting
    //let result = float_third_two / unsigned_eight;
    // error[E0277]: cannot divide `f32` by `u8`
    //    --> src/main.rs:132:34
    //     |
    // 132 |     let result = float_third_two / unsigned_eight;
    //     |                                  ^ no implementation for `f32 / u8`
    //     |
    //     = help: the trait `Div<u8>` is not implemented for `f32`
    //     = help: the following other types implement trait `Div<Rhs>`:
    //               <&'a f32 as Div<f32>>
    //               <&f32 as Div<&f32>>
    //               <f32 as Div<&f32>>
    //               <f32 as Div>

    let cast_unsigned_eight = unsigned_eight as f32;
    let result = float_third_two / cast_unsigned_eight;
    println!("Division is {}", result);

    // Rust can translate the unicode character value
    let number: u8 = 65;
    let letter: char = number as char;
    println!("Char is {}", letter);
}

fn variable_scope_and_shadowing() {
    let scope_test = "outer scope";
    println!("{}", scope_test);
    {
        // variables in the inner scope can be re-declared
        // re-declaration is called shadowing
        // https://www.reddit.com/r/rust/comments/xx6ibp/what_is_the_logic_behind_shadowing/
        let scope_test = "inner scope";
        println!("{}", scope_test);
    }
    println!("{}", scope_test);

    // https://www.reddit.com/r/rust/comments/xx6ibp/what_is_the_logic_behind_shadowing/
    // changing the type of a variable in the same scope
    let a = "String";
    let a: f32 = 5.6;
}

fn language_operators() {
    // OPERATORS
    let modules = 18 % 7;
    println!("{}", modules);

    // the code you for exponent of a floating number is different than to the integer
    // An integer can only have an integer exponent i32::pow(x, y)
    let squared = i32::pow(8, 2);
    println!("{}", squared);

    // A floating-point number can have either an integer of a floating-point exponent
    // f32::powi(x, y) or f32::powf(x, y)
    // In Rust they are separate because it is more expensive to calculate the latter form
    // Power of floating-points are calculated via logarithms
    let float_integer = f32::powi(6.5, 3);
    let float_float = f32::powf(6.5, 3.14);
    println!("integer: {}, float: {}", float_integer, float_float);

    // ORDER of operations
    // () parenthesis, X^2 (exponents), * % Multiplication/Division, +- Addition/subtraction
    // Follows the same order
    let order_ops = 8 + 4 * 2 - (12 / 3 + 7) + 4;
    println!("{}", order_ops);

    // Boolean operators
    //== equals
    // != not equals
    let equals = 1 == 1;
    let n_equals = 1 == 4;
    let n_equals_too = 1 != 4;

    // https://www.reddit.com/r/rust/comments/w2nxrw/snake_case_vs_camel_case/
    let is_true = true;
    let not_true = !is_true;
    println!("is true {}, is false {}", is_true, not_true);

    let have_boarding_pass = true;
    let mut have_id = true;
    let can_board = have_boarding_pass && have_id;
    println!("Can board the plane? {}", can_board);

    // mutable values with mut
    have_id = false;
    println!("Can board the plane? {}", can_board);

    // >, <, >=, <=

    // Bitwise Operators
    // Same as Java &(and), |(or), ^(xor), <<(shift)
    // 01010110 & 00011011 = 00010010 (both)
    println!("bitwise and {}", 86 & 27);
    // 01010110 | 00011011 = 01011111 (one of)
    println!("bitwise or {}", 86 | 27);
    // 01010110 ^ 00011011 = 01001101 (both are different)
    println!("bitwise xor {}", 86 ^ 27);

    // << shift operator is multiplying a number by 2, 4, 8, 16, etc
    // 01010110 << 00011011 = 010011010
    println!("bitwise left shift multiplication {}", 86 << 1);

    // >> shift operator is dividing a number by 2, 4, 8, 16, etc
    // 01010110 << 00011011 = 010011010
    println!("bitwise right shift division  {}", 86 >> 1);
}

// HAVERSINE FORMULA
fn calculate_distance_between_airports() {
    const EARTH_RADIOS_IN_KILOMETERS: f64 = 6371.0;

    let kcle_lat_degrees: f64 = 41.4075;
    let kcle_lon_degrees: f64 = -81.851111;

    let kslc_lat_degrees: f64 = 40.7861;
    let kslc_lon_degrees: f64 = -111.9822;

    let kcle_lat_radians = kcle_lat_degrees.to_radians();
    let kslc_lat_radians = kslc_lat_degrees.to_radians();
    // https://doc.rust-lang.org/std/

    let delta_lat = (kcle_lat_degrees - kslc_lat_degrees).to_radians();
    let delta_lon = (kcle_lon_degrees - kslc_lon_degrees).to_radians();

    let inner_central_angle = f64::powi((delta_lat / 2.0).sin(), 2)
        + kcle_lat_radians.cos()
        * kslc_lat_radians.cos()
        * f64::powi((delta_lon / 2.0).sin(), 2);

    let central_angle = 2.0 * inner_central_angle.sqrt().asin();
    let distance = EARTH_RADIOS_IN_KILOMETERS * central_angle;
    println!("The distance between two points is {:.1} kolometers", distance);
}

fn control_flow_ifs() {
    let word = "Duck";
    if word == "Duck" {
        println!("Quack");
    } else if word == "Dog" {
        println!("Auau");
    } else {
        println!("Super quiet!");
    }

    let a = 10;
    let min = 18;
    if a > 0 && min < 10 {
        println!("We should!")
    }
}

fn enumerations_with_enum() {
    // indexes are sequencial
    enum NavigationAids {
        NDB = 5,
        VOR, // 4
        VORDME, // 5
    }
    enum Colors {
        BLUE, // 0
        GREEN = 3, // 3
        RED // 4
    }
    // indexes can also be objects
    enum Types {
        AIRPORT {name: String, lat: f32, lon: f32},
        ADDRESS {street: String, number: i32, zip: i16, country: i8}
    }

    println!("NDB:\t{}", NavigationAids::NDB as u8);
    println!("VOR:\t{}", NavigationAids::VOR as u8);
    println!("VORDME:\t{}", NavigationAids::VORDME as u8);
}

fn rust_has_no_null_data_type() {
    // causes memory leaks
    // we use Option instead, with 2 methods: some(), none()
    let phrase = String::from("Duck Airlines");
    let letter = phrase.chars().nth(6);

    // Option is a type that has Some or None
    let value = match letter {
        Some(character) => character.to_string(),
        None => String::from("No value")
    };
    println!("Value is '{}'", value);

    let animal = "Duck";
    match animal {
        // arrow pointing is called the arm of the match
        "Duck" => println!("Quack"),
        "Dog" => println!("Bark"),
        _ => println!("All quiet here!")
    };

    let ndb_freq: u16 = 384;
    match ndb_freq {
        // ranges betwee 200 and 500 inclusive
        200..=500 => {
            println!("NDB frequency is valid");
            println!("We can place more stuff...")
        },
        _ => println!("NDB frequency is invalid!")
    }

    match ndb_freq {
        // we can use the if else statement as well
        ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
            println!("NDB frequency is valid");
            println!("We can place more stuff...")
        },
        _ => println!("NDB frequency is invalid!")
    }
}

enum NavigationAids {
    NDB(u16),
    VOR(String, f32),
    VORDME(String, f32),
    FIX{name: String, lat: f32, lon: f32}
}

fn work_with_options_enums() {
    let ndb_uwl = NavigationAids::NDB(385);
    let vor_dqn = NavigationAids::VOR(String::from("DQN"), 114.5);
    let vor_dme_sgh = NavigationAids::VORDME(String::from("SGH"), 113.2);
    let fix_terry = NavigationAids::FIX {
        name: String::from("TARRY"),
        lat: 40.053333,
        lon: -83.91367
    };

    print_nav_aid(&ndb_uwl);
    print_nav_aid(&vor_dqn);
    print_nav_aid(&vor_dme_sgh);
    print_nav_aid(&fix_terry);
}

// & is borrowing
fn print_nav_aid(navigation_aid: &NavigationAids) {
    match navigation_aid {
        // variable that receives the value
        NavigationAids::NDB(khz) => {
            println!("NDB frequency is '{}' kilohertz", khz);
        }
        NavigationAids::VOR(id, freq) => {
            println!("VOR identifier is '{}' and its frequency is {} kilohertz", id, freq);
        }
        NavigationAids::VORDME(id, freq) => {
            println!("VORDME identifier is '{}' and its frequency is {} kilohertz", id, freq);
        }
        NavigationAids::FIX {name, lon, lat} => {
            println!("FIX '{}' is at '{},{}'", name, lon, lat);
        }
    }
}

fn if_let_compiler_macro() {
    let animal = "Duck";

    // compiler will run refutability test, if the value can or cannot different than the current value
    // if so, the value is added into the macro
    // If the value is different, than it's called refutable
    // the operation returns a configuration attribute that evaluates if the let statement is irrefutable
    // the compiler will only include this code if the statement is irrefutable, that's why we use the single equals sign
    if let animal = "Duck" {
        println!("Quack Quack")
    }

    // at compile time, animal will always be "Duck". There's absolutely no way for it to be anything else.
    // so, it's irrefutable, and the println macro will be included in the compiled code.
}

fn while_for_loop_loops() {
    // this is an infinite loop
    // loop {
    //     println!("Hello")
    // }

    // the way to break from the loop is to just break as usual
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 {
            continue;
        }
        if counter != 5 {
            println!("{}", counter);
        }
        if counter == 10 {
            break;
        }
    }

    // while loops
    let mut counter = 1;
    while counter <= 10 {
        println!("{}", counter);
        counter += 1;
    }

    // for loops over collections
    for index in 1..=10 {
        println!("The index is '{}'", index);
    }

    // Iterator Trait, it's like an interface
    // trait Iterator {
    //     type  Item;
    //     fn next(&mut self) -> Option<Self::Item>;
    // }

    let duck_aircrafts = ["Boeing 737", "Boeing 767", "Boeing 787", "Airbus 319", "Airbus 320"];

    // arrays have the Trait "iter()", which returns a collection
    for aircraft in duck_aircrafts.iter() {
        println!("Aircraft is '{}'", aircraft);
    }
}

fn calculate_distance_between_airports_v2() {
    const EARTH_RADIOS_IN_KILOMETERS: f64 = 6371.0;

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
                let previous_waypoint_radians = previous_value.1.to_radians();
                let waypoint_radians = waypoint.1.to_radians();

                let delta_lat = (previous_value.1 - waypoint.1).to_radians();
                let delta_lon = (previous_value.2 - waypoint.2).to_radians();

                let inner_central_angle = f64::powi((delta_lat / 2.0).sin(), 2)
                    + previous_waypoint_radians.cos()
                    * waypoint_radians.cos()
                    * f64::powi((delta_lon / 2.0).sin(), 2);

                let central_angle = 2.0 * inner_central_angle.sqrt().asin();
                let distance = EARTH_RADIOS_IN_KILOMETERS * central_angle;
                total_distance += distance;

                // set the current as previous
                previous_waypoint = Option::from(waypoint.clone());
                println!("The distance between '{}' and '{}' is '{:.1}' kolometers",
                         previous_value.0, waypoint.0, distance);
            }
        }
    }
}

fn main() {
    data_types_and_variables();

    // https://doc.rust-lang.org/book/ch04-03-slices.html
    strings_and_string_slices();

    // https://doc.rust-lang.org/std/
    numbers_variables();

    variable_scope_and_shadowing();

    language_operators();

    calculate_distance_between_airports();

    control_flow_ifs();

    enumerations_with_enum();

    rust_has_no_null_data_type();

    work_with_options_enums();

    if_let_compiler_macro();

    while_for_loop_loops();

    calculate_distance_between_airports_v2();
}
