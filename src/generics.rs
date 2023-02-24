
pub mod Generics {

    use std::ops::{Add, Mul};

    pub fn generics_for_structs() {
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
    }

    pub fn generics_for_lifetime() {
        // Lifetime and Generic: How to use both a lifetimes and Generic types
        // * Separate
        fn lifetime<'a>(p1: &'a i32) {}
        fn generic<T>(p1: T) {}

        // * Both
        //fn both<'a, T>(p1: T, p2 &'a i32){};
    }

    pub fn generics_contraints() {
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
}