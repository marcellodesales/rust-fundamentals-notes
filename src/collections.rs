
pub mod Collections {

    use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

    pub fn collections_support_sequences() {
        println!("Seeing flights using sequences");

        // Arrays and tuples, but without fixed time
        // Sequences, Maps, and Sets
        // Vector, Double Ended Queue Vector

        // Vectors are just a single list of elements, initialized by generics type or macro
        let mut flights:Vec<&str> = Vec::new();
        let mut cars = vec!["sedan", "pickup", "utility"];

        // Operations are similar to other languages: push(), pop(), iter(), remove(), insert()
        flights.push("DA113\tto Boston departs at 06:20");
        flights.push("DA98\tto London departs at 09:43");
        flights.push("DA428\tto Salt Lake City departs at 12:05");
        flights.push("DA41\tto Berlin departs at 15:30");
        flights.push("DA2815\tto Nashville departs at 17:11");

        // iterating over the vector
        for flight in flights.iter() {
            println!("{}", flight);
        }

        // faster method to access, but dangerous because of the potential index out of bounds
        let third = flights[2];
        println!("Third flight: {}", third);

        //let non_existent = flights[10];
        //println!("Third flight: {}", non_existent);
        // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:1046:24
        // stack backtrace:
        //    0:        0x10d5b8b46 - std::backtrace_rs::backtrace::libunwind::trace::hd92f1ceda090ce71
        // ...
        //                                at /Users/marcellodesales/dev/github.com/marcellodesales/rust-fundamentals/src/main.rs:1046:24
        //   20:        0x10d596109 - rust_fundamentals::collections_support_sequences_maps_sets::hf7604db80c277f4b
        //                                at /Users/marcellodesales/dev/github.com/marcellodesales/rust-fundamentals/src/main.rs:1018:5
        //   21:        0x10d596568 - rust_fundamentals::main::h0d9d1e890ed67206

        // Better alternative, slower, but safer is to use get because it returns an Option
        let fifth = flights.get(10);
        match fifth {
            Some(flight) => {
                println!("Flight exists: {}", flight);
            }
            _ => {
                println!("Can't find flight at 10th location");
            }
        }

        // We can use the if let statement to do something with the data...
        if let Some(flight_value) = flights.get(4) {
            println!("4th flight found: {}", flight_value);
        }

        // Adding a new flight between flights is also possible!
        flights.insert(2, "DA918\tto Orlando departs at 11:12");

        // iterating over the vector
        for flight in flights.iter() {
            println!("{}", flight);
        }

        // Canceling a flight, we just remove the element
        flights.remove(1);

        // iterating over the vector
        for flight in flights.iter() {
            println!("{}", flight);
        }
    }

    pub fn collections_support_queues() {
        println!("Seeing flights using queues");
        // Vector Double Ended Queue can be used to add and remove from the front or from the back
        // We also can't sort this collection with a VecDeque
        // This collection is located at "use std::collections::{VecDeque};"
        let mut flights:VecDeque<&str> = VecDeque::new();

        // Operations are similar to other languages: push(), pop(), iter(), remove(), insert()
        flights.push_front("DA918\tto Orlando departs at 11:12");
        flights.push_back("DA428\tto Salt Lake City departs at 12:05");
        flights.push_front("DA98\tto London departs at 09:43");
        flights.push_front("DA113\tto Boston departs at 06:20");
        flights.push_back("DA41\tto Berlin departs at 15:30");
        flights.push_back("DA2815\tto Nashville departs at 17:11");

        // iterating over the vector
        for flight in flights.iter() {
            println!("{}", flight);
        }

        println!("Size of flights: {}", flights.len());
        // Note that contains receives a pointer to a string slice
        println!("Size of flights: {}", flights.contains(&"DA41\tto Berlin departs at 15:30"));
        // remove all elements
        flights.clear();
        println!("Size of flights after clearing: {}", flights.len());

        // NOTE: Mutable methods in the collections
        // iter_mut() iterates over a collection and allows changing of those values, iter() doesn't!
    }

    pub fn collections_support_maps() {
        println!("Seeing flights using maps");

        // maps are structured in a different way:
        // Sequence collections stores entries in a list sequentially
        // Sequences has a single generic data type for the entries (Vec<T> as generic)
        // Map Collections, on the other hand, stores enties as key value pairs
        // Map collections has two generic data types (for key and values) (HashMap<T, U> as generic)
        // HashMap and Btree Map (sorted keys)

        // NOTE: Default Rust collections do NOT have key collision checking! Values are overwritten!
        // Maps come from "use std::collections::HashMap;
        let mut flights = HashMap::new();
        flights.insert("DA918", ("11:12", "Orlando"));
        flights.insert("DA428", ("12:05", "Salt Lake City"));
        flights.insert("DA98", ("09:43", "to London"));
        flights.insert("DA113", ("06:20", "Boston"));
        flights.insert("DA41", ("15:30", "Berlin"));
        flights.insert("DA2815", ("17:11", "Nashville"));

        let flight_number = "DA2815";
        let flight_option = flights.get(flight_number);
        // destruction of the value using unwrap for a tuple
        let (time, destination) = flight_option.unwrap();
        println!("Flight {} departs at {} from {}", flight_number, time, destination);

        if !flights.contains_key(flight_number) {
            flights.insert(flight_number, ("12:00", "Puerto Rico"));

        } else {
            println!("Flight {} already exists!", flight_number);
        }
    }

    pub fn collections_support_sets() {
        println!("Seeing flights using sets");

        // Sets are hybrid between Sequences and Maps.
        // Sets store a value only, but uses a Map to store data internally.
        // However, it uses the value to generate a key; HashSet (unsorted) or BtreeSet (sorted).
        // Sets don't allow duplicates, and the insertion order is not maintained

        let mut flights = HashSet::new();
        flights.insert(("DA918", "11:12", "Orlando"));
        flights.insert(("DA428", "12:05", "Salt Lake City"));
        flights.insert(("DA428", "12:05", "Salt Lake City"));
        flights.insert(("DA113", "06:20", "Boston"));

        // iterating over the vector
        for flight in flights.iter() {
            // we can't print using {} because it doesn't implement Display
            // So, we use {:?} to print the object to string
            println!("{:?}", flight);
        }

        let mut flights = BTreeSet::new();
        flights.insert(("DA918", "11:12", "Orlando"));
        flights.insert(("DA428", "12:05", "Salt Lake City"));
        flights.insert(("DA428", "12:05", "Salt Lake City"));
        flights.insert(("DA113", "06:20", "Boston"));

        // iterating over the vector
        for flight in flights.iter() {
            // we can't print using {} because it doesn't implement Display
            // So, we use {:?} to print the object to string
            println!("{:?}", flight);
        }
    }
}