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

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Error, Read, Write};
use std::ops::{Add, Mul};

//  Split modules https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
pub mod data_types_and_variables;
use crate::data_types_and_variables::DataTypesAndModules;

pub mod strings_and_slices;
use crate::strings_and_slices::StringsAndStringSlices;

pub mod number_variables;
use crate::number_variables::NumbersAndVariables;

pub mod language_operators;
use crate::language_operators::LanguageOperators;

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

fn ownership_and_borrowing() {
    // at this point, the string is at the heap
    let original = String::from("original value");
    println!("\nOriginal: \t\"{}\"", original);

    // let next = original;
    // println!("{}", original);
    //error[E0382]: borrow of moved value: `original`
    //    --> src/main.rs:526:20
    //     |
    // 522 |     let original = String::from("original value");
    //     |         -------- move occurs because `original` has type `String`, which does not implement the `Copy` trait
    // ...
    // 525 |     let next = original;
    //     |                -------- value moved here
    // 526 |     println!("{}", original);
    //     |                    ^^^^^^^^ value borrowed here after move
    //     |
    //     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    // help: consider cloning the value if the performance cost is acceptable
    //     |
    // 525 |     let next = original.clone();
    //     |                        ++++++++

    // EXPLANATION: "original" is a String type, which can change size, and therefore is placed in the heap with a pointer to it in the stack.
    // When we declare "next" it is pointing to original, we are handing over the ownership of "original" to "next".
    // In turn, it gets rid of the "original" variable so it no longer exists.
    // It's a different approach to any other language:

    // ALTERNATIVES:
    // 1. use the clone() method, but we duplicate the memory allocation for the value
    // 2. borrowing is another option: allows another variable to take temporary ownership of data without deallocating the original variable
    // We add the operator &variable
    let next = &original;
    println!("next: {}", next);

    // It works here because next is a read-only pointer to the memory.
    // "original" still owns the memory, but "next" can read the value at the memory address that original is pointing to.

    // let mut mutable_original = String::from("Mutable Original");
    // println!("Original: {}", mutable_original);
    // mutable_original = String::from("new value");
    //
    // let next_mutable = &mutable_original;
    // println!("original: {}", mutable_original);

    // Defining a new scope can provide the borrowing
    // Scope can be {}, if {}, etc

    let mut mutable_original = String::from("Mutable Original");
    println!("Outer Original: {}", mutable_original);

    {
        let next_mutable = &mutable_original;
        println!("Inner scope next: {}", next_mutable);
        println!("inner scope Original: {}", mutable_original);

    } // from this line, next_mutable is out of scope and ownership returns to mutable_original

    println!("outer original val: {}", mutable_original);

    // to change the value, we need to de-reference the value
    {
        let next = &mut mutable_original;
        // can't change the value, because next is a pointer to the memory address of &mut
        //next = String::from("New value");
        *next = String::from("New value directly in the memory");
        // the value in the memory original will change because the memory location is in the heap
        println!("Inner scope next: {}", next);
        println!("inner scope Original: {}", mutable_original);
    }
    println!("outer original val: {}", mutable_original);

    // Lifetimes is used in to show dangling references
    // let outter_scope;
    // {
    //     let inner_scope = 5;
    //     outter_scope = &inner_scope;
    // }
    // println!("{}", outter_scope);
    // error[E0597]: `inner_scope` does not live long enough
    //    --> src/main.rs:597:24
    //     |
    // 596 |         let inner_scope = 5;
    //     |             ----------- binding `inner_scope` declared here
    // 597 |         outter_scope = &inner_scope;
    //     |                        ^^^^^^^^^^^^ borrowed value does not live long enough
    // 598 |     }
    //     |     - `inner_scope` dropped here while still borrowed
    // 599 |     println!("{}", outter_scope);
    //     |                    ------------ borrow later used here

    //let returned = return_bad_ref();
    let referenced_int = 6;
    let referenced_value = return_one_parameter(&referenced_int);
    println!("Referenced param: {}", referenced_value);

    let value_one = 24;
    let value_two = 67;
    let val = explicity_lifetime(&value_one, &value_two);

}

// fn return_bad_ref() -> &i32 {
//     let val = 5;
//     &val;
//     //can be used as a last stagement to return the value
//     //return &val;
//
//     // This method doesn't compile because of the value val goes out of scope at the end of the function.
// }
// error[E0106]: missing lifetime specifier
//    --> src/main.rs:615:24
//     |
// 615 | fn return_bad_ref() -> &i32 {
//     |                        ^ expected named lifetime parameter
//     |
//     = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// help: consider using the `'static` lifetime
//     |
// 615 | fn return_bad_ref() -> &'static i32 {
//     |                         +++++++

//  Lifetime parameter is like generics
// Parameters are pass by reference
// P2 doesn't have an associated lifetime
// Since it's a parameter not passed by reference but rather passed in by value, a lifetime doesn't apply to it
fn lifetime_syntax<'a, 'b>(p1: &'a i32, ps2: i32, p3: &'b f64) {

}

// Since the reference is passed and returned, we don't need a lifetime parameter to the method
fn return_one_parameter(value: &i32) -> &i32 {
    value
}

// the lifetime param applies to both params and the result
// the end of the scope both goes away. So, we tell the compiler to use the lifetime param for both
// params, as well as the lifetime param for the returned value.
// Lifetime only refers to references, and the underlying data that the reference points to has to remain in scope
// otherwise it's a dangling reference.
fn explicity_lifetime<'a>(p1: &'a i32, p2: &'a i32) -> &'a i32 {
    if p1 > p2 {
        p1
    } else {
        p2
    }
}

// param types, and return type with ->, can be omitted if not returning
// return can be omitted and without ;
fn return_greater(first: u8, second: u8) -> u8 {
    if first >= second {
        first
    } else {
        second
    }
}

fn calling_functions() {
    // Pass by Value is when the & is not provided, it's a copy of the value in the stack
    let greater = return_greater(10, 5);
    println!("{}", greater);

    // Pass by reference is passing the pointer
    // borrowing as part of functions
    let mut original = String::from("Mutable Original");
    println!("Outter Original: \t'{}'", original);

    {
        print_original(&original);
        change_original(&mut original);
        println!("inner scope of original: \t'{}'", original);
    }
}

fn closure_methods() {
    let name = "Duck Airlines";
    let write_message_closure = || {
        println!("Hey this is a closure: {}", name);
    };

    // calling a closure
    write_message_closure();

    // it goes between the pipes
    let write_message_closure_with_params = |slogan: String| {
        println!("closure {}. {}", name, slogan);
    };

    // calling closure with param
    write_message_closure_with_params(String::from("We hit the ground every time."));

    let write_message_and_return = |slogan: String| -> String {
        // since this is the return, it's without the semi-colon ;
        String::from(format!("{}. {}", name, slogan))
    };
    let phrase = write_message_and_return(String::from("Phease: We hit again!!!"));
    println!("{}", phrase)
}

// sending the memory address rather than the actual value
fn print_original(original: &String) {
    println!("fn print_original: \t'{}'", original);
}

fn change_original(original: &mut String) {
    let next = original;
    *next = String::from("changed value!!!");
    // The next value MUST MUST MUST be used here as it borrowed the ownership from original pointer
    // The ownership is changed until the exit of the function, which is the scope of that function
    println!("fn change_original: \t'{}'", next);
}

fn unrecoverable_error() {
    let vector = vec![1, 2, 3, 4, 5];
    println!("{}", vector[10]);
    // MUST SET ENV VAR RUST_BACKTRACE=full or RUST_BACKTRACE=1
    //   0:        0x10607dbe6 - std::backtrace_rs::backtrace::libunwind::trace::hd92f1ceda090ce71
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
    //    1:        0x10607dbe6 - std::backtrace_rs::backtrace::trace_unsynchronized::hcf05149d2662e04f
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
    //    2:        0x10607dbe6 - std::sys_common::backtrace::_print_fmt::hcb3d5ccc45542308
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/sys_common/backtrace.rs:65:5
    //    3:        0x10607dbe6 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1bf6b9f14b233587
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/sys_common/backtrace.rs:44:22
    //    4:        0x10609581a - core::fmt::write::h06d5dce650bd77b4
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/fmt/mod.rs:1232:17
    //    5:        0x10607bb1c - std::io::Write::write_fmt::h8ebb8faf14d5cb51
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/io/mod.rs:1684:15
    //    6:        0x10607d9ca - std::sys_common::backtrace::_print::h0b631a2de1b47a1c
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/sys_common/backtrace.rs:47:5
    //    7:        0x10607d9ca - std::sys_common::backtrace::print::hfbc65cc6ee6c9b74
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/sys_common/backtrace.rs:34:9
    //    8:        0x10607efb3 - std::panicking::default_hook::{{closure}}::hd1b6a465fbed8edd
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:271:22
    //    9:        0x10607ed08 - std::panicking::default_hook::h6aeaf6d5284e4708
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:290:9
    //   10:        0x10607f59b - std::panicking::rust_panic_with_hook::h1df136699cf9248d
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:692:13
    //   11:        0x10607f4b4 - std::panicking::begin_panic_handler::{{closure}}::h2c310b4a02e33c0f
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:583:13
    //   12:        0x10607e029 - std::sys_common::backtrace::__rust_end_short_backtrace::h62615855b906167c
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/sys_common/backtrace.rs:137:18
    //   13:        0x10607f17d - rust_begin_unwind
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:579:5
    //   14:        0x10609dbf3 - core::panicking::panic_fmt::h81b0ae56717c49a9
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/panicking.rs:64:14
    //   15:        0x10609dd46 - core::panicking::panic_bounds_check::h0570c1ed1da340e8
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/panicking.rs:159:5
    //   16:        0x10605eaf6 - <usize as core::slice::index::SliceIndex<[T]>>::index::ha919b4532a5e772c
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/slice/index.rs:260:10
    //   17:        0x10605e8d6 - core::slice::index::<impl core::ops::index::Index<I> for [T]>::index::hde64a96e0993fef3
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/slice/index.rs:18:9
    //   18:        0x10605e8d6 - <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index::hb6796975a4cb1552
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/alloc/src/vec/mod.rs:2703:9
    //   19:        0x10605be46 - rust_fundamentals::unrecoverable_error::h9f1531aecc4b7dcb
    //                                at /Users/marcellodesales/dev/github.com/marcellodesales/rust-fundamentals/src/main.rs:736:20
    //   20:        0x10605bf39 - rust_fundamentals::main::h0d9d1e890ed67206
    //                                at /Users/marcellodesales/dev/github.com/marcellodesales/rust-fundamentals/src/main.rs:774:5
    //   21:        0x1060541ee - core::ops::function::FnOnce::call_once::h3c7f52b69edf803f
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/ops/function.rs:250:5
    //   22:        0x10605e981 - std::sys_common::backtrace::__rust_begin_short_backtrace::h296e4579310ffcc6
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/sys_common/backtrace.rs:121:18
    //   23:        0x106056ac4 - std::rt::lang_start::{{closure}}::h25e8adc3cc69b121
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/rt.rs:166:18
    //   24:        0x106079200 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h21844b05036a7397
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/ops/function.rs:287:13
    //   25:        0x106079200 - std::panicking::try::do_call::ha905ecad7031c999
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:487:40
    //   26:        0x106079200 - std::panicking::try::h908fa9e0792f0a82
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:451:19
    //   27:        0x106079200 - std::panic::catch_unwind::h596d02efe9e6f81d
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panic.rs:140:14
    //   28:        0x106079200 - std::rt::lang_start_internal::{{closure}}::he5f89bb7ee56e086
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/rt.rs:148:48
    //   29:        0x106079200 - std::panicking::try::do_call::h42d4b7699af56282
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:487:40
    //   30:        0x106079200 - std::panicking::try::hcf757b5064a6477a
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:451:19
    //   31:        0x106079200 - std::panic::catch_unwind::h7912a5932b004968
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panic.rs:140:14
    //   32:        0x106079200 - std::rt::lang_start_internal::h4f6d8f6e580c2d8d
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/rt.rs:148:20
    //   33:        0x106056a97 - std::rt::lang_start::hfc9299bb388521bd
    //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/rt.rs:165:17
    //   34:        0x10605bf58 - _main
    //   35:     0x7ff80b6bb310 - <unknown>
}

fn handle_error_recoverable_errors() {
    // Rust Prelude is the basics of Rust to get it working
    let file_name = "/tmp/data.json";
    open_file(file_name);
    // Os {
    //     code: 2,
    //     kind: NotFound,
    //     message: "No such file or directory",
    // }
    match write_to_file_with_propagation(file_name) {
        Ok(_) => {
            println!("Wrote to file {}", file_name);
        }
        Err(error) => {
            println!("Couldn't write to file {}", file_name);
            println!("{:#?}", error);
        }
    }

    // Now that we have provided propagation of errors, we can call the method with match!
    match read_file_with_propagation(file_name) {
        // Return the match of the result
        Ok(data) => {
            println!("From file: '{}'", data);
        }
        Err(error) => {
            println!("{:#?}", error);
        }
    }
}

fn open_file(file_name: &str) {
    match File::open(file_name) {
        Ok(file) => {
            println!("{:#?}", file);
        }
        Err(error) => {
            // Since the file might not exist, then we need to handle the error
            println!("{:#?}", error);
            match error.kind() {
              ErrorKind::NotFound => {
                  match File::create(file_name) {
                      Ok(file) => {
                          println!("Created file {}", file_name);
                      }
                      Err(error) => {
                          println!("Couldn't create file at {}. Missing permission?", file_name);
                          println!("{:#?}", error);
                      }
                  }
              }
                _ => {
                    // as we don't have other options
                    println!("{:#?}", error);
                }
            }
        }
    }
}

fn write_to_file_with_propagation(filename: &str) -> Result<(), Error> {
    // https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x/66484174#66484174
    // the ? after the method call is to abort the method execution and throw the error from here
    let mut file_handler = OpenOptions::new()
        .append(true)
        .open(filename)?;

    // as the method can be error, let's just add the ? into the method
    file_handler.write_all(b"Marcello is here!")?;

    Ok(())
}

fn read_file_with_propagation(filename: &str) -> Result<String, Error> {
    // the ? after the method call is to abort the method execution and throw the error from here
    let mut file_handler = File::open(filename)?;
    let mut file_data = String::new();

    // as the method can be error, let's just add the ? into the method
    file_handler.read_to_string(&mut file_data)?;

    // return the 0k only as the handling can be done in other higher-order functions
    Ok(file_data)
}

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

/**
 * Initialization of structs can be used with params
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

fn waypoint_data_structures_with_associated_data() {
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
    let kcle_kslc = Segment::new(kcle, kslc);
    let distance = kcle_kslc.distance();
    println!("Distance from kcle to kslc is '{:.1}' kilometers", distance)
}

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

fn traits_for_structs_and_impl() {
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

fn collections_support_sequences_maps_sets() {
    collections_support_sequences();
    collections_support_queues();
    collections_support_maps();
    collections_support_sets();
}

fn collections_support_sequences() {
    println!("Seeing flights using sequences");

    // Arrays and tuples, but without fixed time
    // Sequences, Maps, and Sets
    // Vector, Double Ended Queue Vector

    // Vectors are just a single list of elements, initialized by generics type or macro
    let mut flights:Vec<&str> = Vec::new();
    let mut cars = vec!["sedan", "pickup", "utility"];

    // Operations are similar to other languages: push(), pop(), iter(), remove(), insert()
    flights.push("DA113\tto Boston departs at 06:20");
    flights.push("DA98\tto London departs at 09:43");
    flights.push("DA428\tto Salt Lake City departs at 12:05");
    flights.push("DA41\tto Berlin departs at 15:30");
    flights.push("DA2815\tto Nashville departs at 17:11");

    // iterating over the vector
    for flight in flights.iter() {
        println!("{}", flight);
    }

    // faster method to access, but dangerous because of the potential index out of bounds
    let third = flights[2];
    println!("Third flight: {}", third);

    //let non_existent = flights[10];
    //println!("Third flight: {}", non_existent);
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:1046:24
    // stack backtrace:
    //    0:        0x10d5b8b46 - std::backtrace_rs::backtrace::libunwind::trace::hd92f1ceda090ce71
    // ...
    //                                at /Users/marcellodesales/dev/github.com/marcellodesales/rust-fundamentals/src/main.rs:1046:24
    //   20:        0x10d596109 - rust_fundamentals::collections_support_sequences_maps_sets::hf7604db80c277f4b
    //                                at /Users/marcellodesales/dev/github.com/marcellodesales/rust-fundamentals/src/main.rs:1018:5
    //   21:        0x10d596568 - rust_fundamentals::main::h0d9d1e890ed67206

    // Better alternative, slower, but safer is to use get because it returns an Option
    let fifth = flights.get(10);
    match fifth {
        Some(flight) => {
            println!("Flight exists: {}", flight);
        }
        _ => {
            println!("Can't find flight at 10th location");
        }
    }

    // We can use the if let statement to do something with the data...
    if let Some(flight_value) = flights.get(4) {
        println!("4th flight found: {}", flight_value);
    }

    // Adding a new flight between flights is also possible!
    flights.insert(2, "DA918\tto Orlando departs at 11:12");

    // iterating over the vector
    for flight in flights.iter() {
        println!("{}", flight);
    }

    // Canceling a flight, we just remove the element
    flights.remove(1);

    // iterating over the vector
    for flight in flights.iter() {
        println!("{}", flight);
    }
}

fn collections_support_queues() {
    println!("Seeing flights using queues");
    // Vector Double Ended Queue can be used to add and remove from the front or from the back
    // We also can't sort this collection with a VecDeque
    // This collection is located at "use std::collections::{VecDeque};"
    let mut flights:VecDeque<&str> = VecDeque::new();

    // Operations are similar to other languages: push(), pop(), iter(), remove(), insert()
    flights.push_front("DA918\tto Orlando departs at 11:12");
    flights.push_back("DA428\tto Salt Lake City departs at 12:05");
    flights.push_front("DA98\tto London departs at 09:43");
    flights.push_front("DA113\tto Boston departs at 06:20");
    flights.push_back("DA41\tto Berlin departs at 15:30");
    flights.push_back("DA2815\tto Nashville departs at 17:11");

    // iterating over the vector
    for flight in flights.iter() {
        println!("{}", flight);
    }

    println!("Size of flights: {}", flights.len());
    // Note that contains receives a pointer to a string slice
    println!("Size of flights: {}", flights.contains(&"DA41\tto Berlin departs at 15:30"));
    // remove all elements
    flights.clear();
    println!("Size of flights after clearing: {}", flights.len());

    // NOTE: Mutable methods in the collections
    // iter_mut() iterates over a collection and allows changing of those values, iter() doesn't!
}

fn collections_support_maps() {
    println!("Seeing flights using maps");

    // maps are structured in a different way:
    // Sequence collections stores entries in a list sequentially
    // Sequences has a single generic data type for the entries (Vec<T> as generic)
    // Map Collections, on the other hand, stores enties as key value pairs
    // Map collections has two generic data types (for key and values) (HashMap<T, U> as generic)
    // HashMap and Btree Map (sorted keys)

    // NOTE: Default Rust collections do NOT have key collision checking! Values are overwritten!
    // Maps come from "use std::collections::HashMap;
    let mut flights = HashMap::new();
    flights.insert("DA918", ("11:12", "Orlando"));
    flights.insert("DA428", ("12:05", "Salt Lake City"));
    flights.insert("DA98", ("09:43", "to London"));
    flights.insert("DA113", ("06:20", "Boston"));
    flights.insert("DA41", ("15:30", "Berlin"));
    flights.insert("DA2815", ("17:11", "Nashville"));

    let flight_number = "DA2815";
    let flight_option = flights.get(flight_number);
    // destruction of the value using unwrap for a tuple
    let (time, destination) = flight_option.unwrap();
    println!("Flight {} departs at {} from {}", flight_number, time, destination);

    if !flights.contains_key(flight_number) {
        flights.insert(flight_number, ("12:00", "Puerto Rico"));

    } else {
        println!("Flight {} already exists!", flight_number);
    }
}

fn collections_support_sets() {
    println!("Seeing flights using sets");

    // Sets are hybrid between Sequences and Maps.
    // Sets store a value only, but uses a Map to store data internally.
    // However, it uses the value to generate a key; HashSet (unsorted) or BtreeSet (sorted).
    // Sets don't allow duplicates, and the insertion order is not maintained

    let mut flights = HashSet::new();
    flights.insert(("DA918", "11:12", "Orlando"));
    flights.insert(("DA428", "12:05", "Salt Lake City"));
    flights.insert(("DA428", "12:05", "Salt Lake City"));
    flights.insert(("DA113", "06:20", "Boston"));

    // iterating over the vector
    for flight in flights.iter() {
        // we can't print using {} because it doesn't implement Display
        // So, we use {:?} to print the object to string
        println!("{:?}", flight);
    }

    let mut flights = BTreeSet::new();
    flights.insert(("DA918", "11:12", "Orlando"));
    flights.insert(("DA428", "12:05", "Salt Lake City"));
    flights.insert(("DA428", "12:05", "Salt Lake City"));
    flights.insert(("DA113", "06:20", "Boston"));

    // iterating over the vector
    for flight in flights.iter() {
        // we can't print using {} because it doesn't implement Display
        // So, we use {:?} to print the object to string
        println!("{:?}", flight);
    }
}

fn generics_for_structs() {
    // Collections and functions can be generalized by using Generics
    // integer vector = Vec<i32>, Vec<&str>, Vec<bool> -> Vec<T>
    // Declaring variables as generics and Constraints

    struct NdbNavAid {
        name: String,
        frequency: u16 // note that this type is different for this struct based on requirements
    }
    struct VorNavAid {
        name: String,
        frequency: f32 // this is the same as well, different
    }

    // The refactored version of this struct can take the T form
    #[derive(Debug)]
    struct NavAid<T, U> {
        name: String,
        frequency: T,
        data: U
    }

    // Without derive(Debug) above, the code won't compile with the following error
    // https://stackoverflow.com/questions/73166648/t-cannot-be-formatted-with-the-default-formatter/73167159#73167159
    // error[E0277]: `NavAid<{float}, String>` doesn't implement `Debug`
    //     --> src/main.rs:1235:34
    //      |
    // 1235 |     println!("VOR info is {:?}", vor);
    //      |                                  ^^^ `NavAid<{float}, String>` cannot be formatted using `{:?}`
    //      |
    //      = help: the trait `Debug` is not implemented for `NavAid<{float}, String>`
    //      = note: add `#[derive(Debug)]` to `NavAid<{float}, String>` or manually `impl Debug for NavAid<{float}, String>`
    //      = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

    let vor = NavAid {
        name: String::from("DQN"),
        frequency: 114.5, // this is a value that is way smaller
        data: String::from("This is my data")
    };
    let ndb_data: Option<String> = Option::None;
    let ndb = NavAid {
        name: String::from("HKF"),
        frequency: 239,
        data: ndb_data
    };

    println!("VOR info is {:?}", vor);
    println!("NDB info is {:?}", ndb);

    // Lifetime and Generic: How to use both a lifetimes and Generic types
    // * Separate
    fn lifetime<'a>(p1: &'a i32){}
    fn generic<T>(p1: T) {}

    // * Both
    //fn both<'a, T>(p1: T, p2 &'a i32){};

    // Constraints limits what we need to accept in the generics
    // whatever data type is passed in has to be able to be multiplied together
    // WITHOUT TRAITS, we will get a failure!

    // fn multiply<T>(operand1: T, operand2: T) -> T { operand1 * operand2 };
    // error[E0369]: cannot multiply `T` by `T`
    //     --> src/main.rs:1256:62
    //      |
    // 1256 |     fn multiply<T>(operand1: T, operand2: T) -> T { operand1 * operand2 };
    //      |                                                     -------- ^ -------- T
    //      |                                                     |
    //      |                                                     T
    //      |
    // help: consider restricting type parameter `T`
    //      |
    // 1256 |     fn multiply<T: std::ops::Mul<Output = T>>(operand1: T, operand2: T) -> T { operand1 * operand2 };

    // COMPILER doesn't know how if the operation can be used for any type.
    // In order to make it work, we need to add a Trait to the generic type.
    // It must be from the std::ops::Add, and must specify the Output type
    // Usually, you will just use the trait name and won't need to add anything in angle brackets
    // like we are doing for the add...
    fn add<T: Add<Output = T>>(operarand1: T, operand2: T) -> T {
        operarand1 + operand2
    }

    let sum = add(12, 34);
    println!("Sum is {}", sum);

    // We can use the the where clause of the generics to specify other traits...
    fn multiply<T>(operand1: T, operand2: T) -> T
    where T: Mul<Output = T> {
        operand1 * operand2
    };

    let mult = multiply(sum, 4);
    println!("Multiplication is {}", mult);

    // This is just an example of multiple traits, this is NOT useful but shows multiple traits!
    fn add_or_multiply<T>(operand1: T, operand2: T) -> T
    where T: Add<Output = T> + Mul<Output = T> {
        operand1 + operand2
    }
}

fn main() {
    // Split modules https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
    DataTypesAndModules::hello_world();
    DataTypesAndModules::arrays();
    DataTypesAndModules::tuplues();

    // https://doc.rust-lang.org/book/ch04-03-slices.html
    StringsAndStringSlices::strings_in_heap();
    StringsAndStringSlices::string_slices_in_stack();
    StringsAndStringSlices::string_heap_pointers();
    StringsAndStringSlices::string_to_string_slice_conversion();

    // https://doc.rust-lang.org/std/
    NumbersAndVariables::signed_and_unsined_integers();
    NumbersAndVariables::floating_point_numbers_and_casting();
    NumbersAndVariables::unicode_numbers();

    variable_scope_and_shadowing();

    LanguageOperators::modulus_operator();
    LanguageOperators::power_operator();
    LanguageOperators::precedence_of_operations();
    LanguageOperators::boolean_operations();
    LanguageOperators::mutable_operations_in_variables();
    LanguageOperators::bitwise_operations();

    calculate_distance_between_airports();

    control_flow_ifs();

    enumerations_with_enum();

    rust_has_no_null_data_type();

    work_with_options_enums();

    if_let_compiler_macro();

    while_for_loop_loops();

    calculate_distance_between_airports_v2();

    ownership_and_borrowing();

    calling_functions();

    closure_methods();

    //unrecoverable_error();

    handle_error_recoverable_errors();

    waypoint_data_structures_with_associated_data();

    traits_for_structs_and_impl();

    collections_support_sequences_maps_sets();

    generics_for_structs();
}
