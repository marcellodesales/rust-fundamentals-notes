
/**
 * Ownership and borrowing are basic concepts
 */
pub mod OwnershipAndBorrowing {

    pub fn moving_ownership() {
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
    }

    pub fn lifetimes_and_dangling_references() {
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
}