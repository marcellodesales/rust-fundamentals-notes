// for unused variables
#![allow(unused_variables)]

// entrypoint of the command, () as regular function
fn main() {
    // ! is a macro and not a function, set of code that has a name
    let unused_variable: u32 = 0;
    println!("Hello, world!");

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


    // https://doc.rust-lang.org/book/ch04-03-slices.html

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



    // Scope and Shadowing

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
    let a = "String";
    let a: f32 = 5.6;
}
