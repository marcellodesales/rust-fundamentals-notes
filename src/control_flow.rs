
pub mod ControlFlow {

    pub fn control_flow_ifs() {
        let word = "Duck";
        if word == "Duck" {
            println!("Quack");
        } else if word == "Dog" {
            println!("Auau");
        } else {
            println!("Super quiet!");
        }

        let a = 10;
        let min = 18;
        if a > 0 && min < 10 {
            println!("We should!")
        }
    }

    pub fn enumerations_with_enum() {
        // indexes are sequencial
        enum NavigationAids {
            NDB = 5,
            VOR, // 4
            VORDME, // 5
        }
        enum Colors {
            BLUE, // 0
            GREEN = 3, // 3
            RED // 4
        }
        // indexes can also be objects
        enum Types {
            AIRPORT {name: String, lat: f32, lon: f32},
            ADDRESS {street: String, number: i32, zip: i16, country: i8}
        }

        println!("NDB:\t{}", NavigationAids::NDB as u8);
        println!("VOR:\t{}", NavigationAids::VOR as u8);
        println!("VORDME:\t{}", NavigationAids::VORDME as u8);
    }

    pub fn rust_has_no_null_data_type() {
        // causes memory leaks
        // we use Option instead, with 2 methods: some(), none()
        let phrase = String::from("Duck Airlines");
        let letter = phrase.chars().nth(6);

        // Option is a type that has Some or None
        let value = match letter {
            Some(character) => character.to_string(),
            None => String::from("No value")
        };
        println!("Value is '{}'", value);

        let animal = "Duck";
        match animal {
            // arrow pointing is called the arm of the match
            "Duck" => println!("Quack"),
            "Dog" => println!("Bark"),
            _ => println!("All quiet here!")
        };

        let ndb_freq: u16 = 384;
        match ndb_freq {
            // ranges betwee 200 and 500 inclusive
            200..=500 => {
                println!("NDB frequency is valid");
                println!("We can place more stuff...")
            },
            _ => println!("NDB frequency is invalid!")
        }

        match ndb_freq {
            // we can use the if else statement as well
            ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
                println!("NDB frequency is valid");
                println!("We can place more stuff...")
            },
            _ => println!("NDB frequency is invalid!")
        }
    }

    enum NavigationAids {
        NDB(u16),
        VOR(String, f32),
        VORDME(String, f32),
        FIX{name: String, lat: f32, lon: f32}
    }

    pub fn work_with_options_enums() {
        let ndb_uwl = NavigationAids::NDB(385);
        let vor_dqn = NavigationAids::VOR(String::from("DQN"), 114.5);
        let vor_dme_sgh = NavigationAids::VORDME(String::from("SGH"), 113.2);
        let fix_terry = NavigationAids::FIX {
            name: String::from("TARRY"),
            lat: 40.053333,
            lon: -83.91367
        };

        print_nav_aid(&ndb_uwl);
        print_nav_aid(&vor_dqn);
        print_nav_aid(&vor_dme_sgh);
        print_nav_aid(&fix_terry);
    }

    // & is borrowing
    fn print_nav_aid(navigation_aid: &NavigationAids) {
        match navigation_aid {
            // variable that receives the value
            NavigationAids::NDB(khz) => {
                println!("NDB frequency is '{}' kilohertz", khz);
            }
            NavigationAids::VOR(id, freq) => {
                println!("VOR identifier is '{}' and its frequency is {} kilohertz", id, freq);
            }
            NavigationAids::VORDME(id, freq) => {
                println!("VORDME identifier is '{}' and its frequency is {} kilohertz", id, freq);
            }
            NavigationAids::FIX {name, lon, lat} => {
                println!("FIX '{}' is at '{},{}'", name, lon, lat);
            }
        }
    }

    pub fn if_let_compiler_macro() {
        let animal = "Duck";

        // compiler will run refutability test, if the value can or cannot different than the current value
        // if so, the value is added into the macro
        // If the value is different, than it's called refutable
        // the operation returns a configuration attribute that evaluates if the let statement is irrefutable
        // the compiler will only include this code if the statement is irrefutable, that's why we use the single equals sign
        if let animal = "Duck" {
            println!("Quack Quack")
        }

        // at compile time, animal will always be "Duck". There's absolutely no way for it to be anything else.
        // so, it's irrefutable, and the println macro will be included in the compiled code.
    }

    pub fn while_for_loop_loops() {
        // this is an infinite loop
        // loop {
        //     println!("Hello")
        // }

        // the way to break from the loop is to just break as usual
        let mut counter = 0;
        loop {
            counter += 1;
            if counter == 3 {
                continue;
            }
            if counter != 5 {
                println!("{}", counter);
            }
            if counter == 10 {
                break;
            }
        }

        // while loops
        let mut counter = 1;
        while counter <= 10 {
            println!("{}", counter);
            counter += 1;
        }

        // for loops over collections
        for index in 1..=10 {
            println!("The index is '{}'", index);
        }

        // Iterator Trait, it's like an interface
        // trait Iterator {
        //     type  Item;
        //     fn next(&mut self) -> Option<Self::Item>;
        // }

        let duck_aircrafts = ["Boeing 737", "Boeing 767", "Boeing 787", "Airbus 319", "Airbus 320"];

        // arrays have the Trait "iter()", which returns a collection
        for aircraft in duck_aircrafts.iter() {
            println!("Aircraft is '{}'", aircraft);
        }
    }

}