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

use std::fmt::Display;
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

pub mod project;
pub mod control_flow;
pub mod ownership_and_borrowing;
pub mod functions_definitions;
pub mod error_handling;
pub mod data_structures_and_traits;
pub mod collections;

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

    project::ProjectV1::calculate_distance_between_airports();

    control_flow::ControlFlow::control_flow_ifs();
    control_flow::ControlFlow::enumerations_with_enum();
    control_flow::ControlFlow::rust_has_no_null_data_type();
    control_flow::ControlFlow::work_with_options_enums();
    control_flow::ControlFlow::if_let_compiler_macro();
    control_flow::ControlFlow::while_for_loop_loops();

    project::ProjectV2::calculate_distance_between_airports();

    ownership_and_borrowing::OwnershipAndBorrowing::moving_ownership();
    ownership_and_borrowing::OwnershipAndBorrowing::lifetimes_and_dangling_references();

    functions_definitions::FunctionsDefinitions::calling_functions();
    functions_definitions::FunctionsDefinitions::closure_methods();

    // error_handling::ErrorHandling::unrecoverable_error();
    error_handling::ErrorHandling::handle_error_recoverable_errors();

    data_structures_and_traits::StructsAndTraits::waypoint_data_structures_with_associated_data();

    project::ProjectV3::traits_for_structs_and_impl();

    collections::Collections::collections_support_sequences();
    collections::Collections::collections_support_queues();
    collections::Collections::collections_support_maps();
    collections::Collections::collections_support_sets();

    generics_for_structs();
}
