
pub mod FunctionsDefinitions {

    // param types, and return type with ->, can be omitted if not returning
// return can be omitted and without ;
    fn return_greater(first: u8, second: u8) -> u8 {
        if first >= second {
            first
        } else {
            second
        }
    }

    pub fn calling_functions() {
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

    pub fn closure_methods() {
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
}