// for unused variables
//#![allow(unused_variables)]

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
}
