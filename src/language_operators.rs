/**
 *
 */
pub mod LanguageOperators {

    pub fn modulus_operator() {
        // OPERATORS
        let modules = 18 % 7;
        println!("{}", modules);
    }

    pub fn power_operator() {
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
    }

    pub fn precedence_of_operations() {
        // ORDER of operations
        // () parenthesis, X^2 (exponents), * % Multiplication/Division, +- Addition/subtraction
        // Follows the same order
        let order_ops = 8 + 4 * 2 - (12 / 3 + 7) + 4;
        println!("{}", order_ops);
    }

    pub fn boolean_operations() {
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
    }

    pub fn mutable_operations_in_variables() {
        let have_boarding_pass = true;
        let mut have_id = true;
        let can_board = have_boarding_pass && have_id;
        println!("Can board the plane? {}", can_board);

        // mutable values with mut
        have_id = false;
        println!("Can board the plane? {}", can_board);
    }

    pub fn bitwise_operations() {
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
}