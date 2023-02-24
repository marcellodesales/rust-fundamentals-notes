# Rust Fundamentals

> **NOTE**: First contrib to Rust repo: https://github.com/rust-lang/mdBook/pull/2030
> * TODO: Revisit the studies with https://doc.rust-lang.org/rust-by-example

Why learn rust? Rust memory management is handled by Rust itself, without the need of a garbage collector.

* C/C++ needs a malloc for memory management
* Java and C# has garbage collector 
* If your Rust code compiles, it will run "without error"
  * Of course, if it doesn't have runtime errors!
* Native cross-platform executables
  * Windows, linux, Mac, IOT, etc

Its memory management helps developers to enforce consistency, which supports governance and makes onboarding easier.
In addition, it allows mentoring of developers to focus on areas other than defensive coding. Even though Rust has 
a steep learning curve, it's the number 1 lang since 2016 even if its commercial use isn't too popular yet.

Programming languages can be categorized as Strongly or Weakly typed, or Static and Dynamic.

```
                                  STRONG
                  Erlang            ^                            Rust
           Ruby                     |                    Scala
           Closure                  |                 Java
         Python                     |            C#        F#
                                    |
DYNAMIC <————————————————————————————————————————————————————————> STATIC
                                    |
                PHP                 |
          Perl                      |            C
                   Javascript       |               C++
           VB                       |
                                   WEAK
```

* **Static**: we know all data types at compile time
* **Dynamic**: We only know the data types at runtime
* **Strong**: enforces rules on data type assignments (OOP)
* **Weak**: Has a fewer to no enforcement on data type assignments (Pointers in C)

For instance, **Strongly-Typed** languages with OOP: Dog and Doc classes implements more types.

* **Compiled Languages**: takes code to machine code
* **Interpreted languages**: interprets the code on-the-fly 
  * Easy to run in hardware, that’s why, write once run anywhere.

# Data Storage in Memory

What is the difference between stack and heap memory?

* Stack memory stores values in the order it gets them and removes them in the opposite order, whereas 
* Heap memory stores values wherever there is free space.

## Stack

* Function calls, local variables in functions
* Frame is placed on the stack when the function is called
* Stack frames are limited, you get stack overflow
* data too big for the stack goes to the heap
* Pointer lives with us in the stack that points to a location in the heap

## Heap

* Data that’s too large goes to the heap
* Pointers address where the data lives in memory
* **Memory addressing: ownership and borrowing concepts of rust**

# Data Types

* Numbers
* Text Data
* Scalar Data Types
* Compound Data Types

## Primitive Data types

Built into the language

* Integer: -1, 0, 1, 2
* Assigned integer (negatives)
* Bits: 8, 16, 32, 64, 128
* Unassigned 8-bit integer
  * u8 = 0..255
* Assigned Integer
  * i8 = -128…127
  * The range is defined because of the 2’s compliment. That is, invert all digits in the binary number and add one!
* Base 10 to Base 2
  * Computers are binary
  * Must be standardized: translating base 10.

## Unsigned 8-bit Binary

```
128	64	32	16	8	4	2	1
0	0	0	0	0	0	0	0
------------------------------------------------------------------
1	1	1	1	1	1	1	1  —> 255
```

# Signed 8-bit Binary (-1)

* Negative numbers have the first bit as the sign

```
SignBit   64  32  16  8   4   2   1
1         1   1   1   1   1   1   1 >>> first bit is the sign, 0 positive 1 negative… this is the -1 … To get the 2’s complement
-------------------------------------------------------
1         0   0   0   0   0   0   1 —> -1 value
```

* Signed +/-:  we use `i` signed integer
* Unsigned: we use `u` unsigned integer
* `isize` and `size`: tied to CPU architecture
  * 32bit CPU, 32bit integer
  * 64bit CPU, 64bit integer

## Floating Point Numbers

* `f32` and `f64` and it’s the same as usual

## Boolean: 

Either `True` or `False`

## Characters

letters in the keyboard, letters and numbers. ASCII and Unicode Tables.

* 2-byte characters in Java
* 1 Byte: 255 characters in ASCII table
* 2 Bytes: 65,535 characters in unicode-16 table
* 4 Bytes: 4,294,967,296 characters in unicode-32 table
  * Rust uses 4 Bytes to represent any language!
  * Rust uses single quotes for characters instead of double-quotes

## Compound Data Types

Holds Multiple Values: Arrays and Tuples

* Arrays: multiple values of a single data type
  * Can’t change its size
  * Located sequential
* Tuples: multiple values but can be different data types

* Example: Location made up of latitude and longitude
  * Degrees - Minutes - Seconds: `41-24-33.8650N 081-51-16.8880W`
  * Degrees: floating point numbers: `41.4094069, -81.8546911`

## Strings and String Slices

* Strings are complex in Rust as compared to many other languages
* This is a trade off that rus has made to support its core principles:
  * Speed
  * Concurrency
  * Memory Safety

* `String`
  * vector of u8 data
  * Mutable
  * Stored on the Heap
  * A String is stored on the heap because it can grow and shrink in size. The size is not constant so it cannot be stored on the stack!
* `&str`
  * Vector of u8 data
  * Immutable
  * Can be stored on the heap, stack or embedded in the compiled code depends on how it is used

# MATH OPERATORS

```
+ - / * % ^
```

# Ownership and Borrowing

* This is the mechanism of which Rust uses to manage memory
* Keep in mind
  * Who owns the data at a given moment in time?
  * Passing by Reference or by Value?
  * Is the data Mutable?

* Only Apply to data on the Heap... There’s a very low cost to copy data in the stack.
* Memory Management in Rust
  * Developer friendly of Garbage Collection of other languages: creating, dealococating
  * .Net events weren’t de-allocated because the classes were’t garbage collected
* Memory Safety
  * Pretty good memory management
  * All the memory management burden is on the developer (C, C++) (Allocating and Deallocating manually)
  * Rust takes responsibility for the memory management  (Rust doesn’t give options)

> **REMEMBER**: 1 and only 1 owner of data at a given time (At a memory location) can exist in Rust!
> * Advantages of Rust: safe and fast because of all the analysis can be done at compile time rather than runtime.
> * The compiler can write optimized code
> * Ownership prevents other issues particularly in the concurrency module.

# Recoverable Error and Unrecoverable Error

* Panic is an Unrecoverable error
* Rust guides you to well-behaved error handling
* Handle all the errors that can happen
* Use Result and Option enumerations where it makes sense
* Error propagation so other functions can handle

Methods and patterns to avoid errors:
* Use SOLID and DRY principles
  * https://www.digitalocean.com/community/conceptual-articles/s-o-l-i-d-the-first-five-principles-of-object-oriented-design
  * https://thevaluable.dev/dry-principle-cost-benefit-example/

# Data Structures, Methods and Traits

* Methods associated
* Traits: similar to OOP Interfaces
  * Associated data structure in Rust uses a struct with a block of impl.
  * Trait is a way to define shared behavior among data

# Concurrent vs Parallel

* Multi-threaded software development involves the design to run multiple tasks at the same time.
* Concurrent processing happens on a single CPU.
  * The OS thread scheduler gives each thread a certain amount of time to execute before
  * it suspends that thread's execution and give its time to a different thread.
* Parallel processing happens on multiple CPUs, multi-cores on a single Processor.
  * NOw conceptually, all threads process on their own core at the same time.
  * Realistically, a thread will not have the undivided attention to a processor.

# Native Threads and Green Threads

* Native Threads: OS Threads 1:1 threads, System threads
  * Managed by the operating system
  * Rust only supports native threads out of the box
* Green Threads: C has coroutines, Go has goroutines, Ruby has fibers,
  * Multiple green threads map to a native thread
  * Entirely managed by your code.
  * End-to-end model where m number of green threads map to n number of OS threads
  * Good at avoiding race conditions!

* Crates.io has the libraries for Rust, including threads
  * https://crates.io/crates/greenie is a green thread package!
* Pain free concurrency of rust. As concurrency errors are caught at compile time, saving 
time and pain when developing concurrent applications.
* Avoid the hazards of concurrent code
* How to create threads
* How threads communicate with each other
* Threads are non-deterministic on when it will run: we don’t know.
* The two main hazards of threads are:
  * Race conditions and deadlocks are a result of the non-deterministic behavior
  * Example of Race Condition: 2 Passengers trying to book a reservation for the same seat at an airline for the same flight!

* Deadlock is when two threads wait in a resource that the other hold before being able to continue processing.
  * Example: two aircrafts is waiting on each other’s resources: the first one is at the gate parked while the other 
    has a crate that is carry it. The former is waiting for the crate to be available while the latter one is in the 
    direction of the gate, which is already occupied by the former aircraft.

# Building packages, modules, crates 

> **NOTE**: A Rust package depends on two crates, A and B, both of which depend on crate C. What happens if crate A 
> depends on C version 1.1.0 and crate B depends on version 2.0.0? 
> * Cargo allows this, but builds two separate copies of the dependency!

> **NOTE**: What is the difference between a Rust package and a crate?
> * A crate is a program that provides some functionality, whereas a package is a wrapper for at least one crate.

## CARGO Modules

* Two files that controls dependencies and how they are managed
* Fetches and builds your package dependencies
* Invokes rust compiler with the correct parameters to build your package!
* Introduces conventions that make working with rust packages easier.
* Creating projects, running tests, continuous integration, documentation generation and publishing packages.

## Code Modules

Cargo files are:

* `Cargo.toml`
  * Cargo commands
  * Publishing packages
  * Not a single file
* Organize modules into different modules
* Break code into multiple files
* File is written in toml language https://toml.io/en/
* https://doc.rust-lang.org/cargo/reference/manifest.html

## Cargo does

* Building the code https://doc.rust-lang.org/cargo/commands/
  * Cargo build
  * Cargo run
  * Cargo clean
  * cargo check of dependencies
  * Cargo doc
  * Cargo rustdoc is like javadoc
  * Cargo test
  * Tests go to the tests dir similar to src
  * Cargo new creates a new project 

## Cargo package

* Crate is a zipped file with the source-code instead of compiled code.
* Lock file is for reproducibility of builds

```
$ cargo package --allow-dirty
warning: manifest has no documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
Packaging rust-fundamentals v0.1.0 (/Users/marcellodesales/dev/github.com/marcellodesales/rust-fundamentals)
Updating crates.io index
Verifying rust-fundamentals v0.1.0 (/Users/marcellodesales/dev/github.com/marcellodesales/rust-fundamentals)
Compiling libc v0.2.139
Compiling cfg-if v1.0.0
Compiling ppv-lite86 v0.2.17
Compiling getrandom v0.2.8
Compiling rand_core v0.6.4
Compiling rand_chacha v0.3.1
Compiling rand v0.8.5
Compiling rust-fundamentals v0.1.0 (/Users/marcellodesales/dev/github.com/marcellodesales/rust-fundamentals/target/package/rust-fundamentals-0.1.0)
warning: unnecessary trailing semicolon
--> src/generics.rs:100:10
|
100 |         };
|          ^ help: remove this semicolon
|
= note: `#[warn(redundant_semicolons)]` on by default

warning: irrefutable `if let` pattern
--> src/control_flow.rs:135:12
|
135 |         if let animal = "Duck" {
|            ^^^^^^^^^^^^^^^^^^^
|
= note: this pattern will always match, so the `if let` is useless
= help: consider replacing the `if let` with a `let`
= note: `#[warn(irrefutable_let_patterns)]` on by default
…
…

warning: `rust-fundamentals` (bin "rust-fundamentals") generated 30 warnings (run `cargo fix --bin "rust-fundamentals"` to apply 1 suggestion)
Finished dev [unoptimized + debuginfo] target(s) in 5.22s
Packaged 20 files, 66.6KiB (18.9KiB compressed)
```

## Private Package Registry

* https://www.jfrog.com/confluence/display/JFROG/Cargo+Package+Registry
* Cargo publish and yank to remove from the registry

# Additional Studies

* Study https://github.com/rust-lang/rustlings
* https://doc.rust-lang.org/rust-by-example
* https://cheats.rs/ shows everything about rust!!!
* Ask questions at https://www.rust-lang.org/community
