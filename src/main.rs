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
pub mod generics;
pub mod concurrency;
pub mod geo;

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
    NumbersAndVariables::variable_scope_and_shadowing();

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

    generics::Generics::generics_for_structs();
    generics::Generics::generics_for_lifetime();
    generics::Generics::generics_contraints();

    concurrency::Concurrency::create_thread();
    concurrency::Concurrency::threads_communication_by_messaging_passing();
}
