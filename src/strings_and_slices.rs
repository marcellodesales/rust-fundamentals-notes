
/**
 * // https://doc.rust-lang.org/book/ch04-03-slices.html

       // What is the difference between stack and heap memory?
       // Stack memory stores values in the order it gets them and removes them in the opposite
       // order, whereas heap memory stores values wherever there is free space.

        error[E0277]: the type `String` cannot be indexed by `{integer}`
        Rust strings do not support indexing.

 */
pub mod StringsAndStringSlices {

    pub fn strings_in_heap() {
        // Strings mutable, stored in the heap because it can grow and shrink in size
        // The size is not constant so it cannot be stored in the stack
        let person_name_string: String = "Donald Mallard".to_string();
        let person_name_string: String = String::from("Donald Mallard");
    }

    pub fn string_slices_in_stack() {
        // String Slices are Immutable, can be stored in heap, stack or embedded in the compiled code
        let person_name_slice: &str = "Donald Mallard";
    }

    pub fn string_heap_pointers() {
        let person_name_string: String = "Donald Mallard".to_string();
        let person_name_string: String = String::from("Donald Mallard");

        // Just using the & in front of the variable to get its pointer memory address where data lives
        // This is the so-called de-referencing
        let person_slice_2: &String = &person_name_string;
    }

    pub fn string_to_string_slice_conversion() {
        let person_name_string: String = String::from("Donald Mallard");

        // Converting String to &str is simple, we know the string is in the heap
        let person_sliced: &str = person_name_string.as_str();
    }

    pub fn string_concatenation_operations() {
        // Concatenation with Strings in rust is complicated
        // Slices are immutable! so we can concatenate
        let duck = "Duck";
        let airlines = "Airlines";

        let airline_name = [duck, " ", airlines].concat();
        println!("Airline name using array concat: {}", airline_name);

        let airline_name = format!("{} {}", duck, airlines);
        println!("Airline name using format: {}", airline_name);
        // can't change the value of immutable strings
        // airline_name = airline_name + "d";

        // mutable strings so it can change
        let mut slogan = String::new();
        // push a string
        slogan.push_str("We hit the gorund");
        // push just a character with single quotes
        slogan.push(' ');
        // concatenate
        slogan = slogan + "every time";
        println!("Our slogan: {}", slogan);
    }
}