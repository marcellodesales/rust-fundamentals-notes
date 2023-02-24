/**
 * Numbers
 */
pub mod NumbersAndVariables {

    pub fn signed_and_unsined_integers() {
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
    }

    pub fn floating_point_numbers_and_casting() {
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
    }

    pub fn unicode_numbers() {
        // Rust can translate the unicode character value
        let number: u8 = 65;
        let letter: char = number as char;
        println!("Char is {}", letter);
    }
}