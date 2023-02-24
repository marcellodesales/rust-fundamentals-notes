
pub mod ErrorHandling {
    use std::fs::{File, OpenOptions};
    use std::io::{ErrorKind, Error, Read, Write};

    pub fn unrecoverable_error() {
        let vector = vec![1, 2, 3, 4, 5];
        println!("{}", vector[10]);
        // MUST SET ENV VAR RUST_BACKTRACE=full or RUST_BACKTRACE=1
        //   0:        0x10607dbe6 - std::backtrace_rs::backtrace::libunwind::trace::hd92f1ceda090ce71
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
        //    1:        0x10607dbe6 - std::backtrace_rs::backtrace::trace_unsynchronized::hcf05149d2662e04f
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
        //    2:        0x10607dbe6 - std::sys_common::backtrace::_print_fmt::hcb3d5ccc45542308
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/sys_common/backtrace.rs:65:5
        //    3:        0x10607dbe6 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1bf6b9f14b233587
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/sys_common/backtrace.rs:44:22
        //    4:        0x10609581a - core::fmt::write::h06d5dce650bd77b4
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/fmt/mod.rs:1232:17
        //    5:        0x10607bb1c - std::io::Write::write_fmt::h8ebb8faf14d5cb51
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/io/mod.rs:1684:15
        //    6:        0x10607d9ca - std::sys_common::backtrace::_print::h0b631a2de1b47a1c
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/sys_common/backtrace.rs:47:5
        //    7:        0x10607d9ca - std::sys_common::backtrace::print::hfbc65cc6ee6c9b74
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/sys_common/backtrace.rs:34:9
        //    8:        0x10607efb3 - std::panicking::default_hook::{{closure}}::hd1b6a465fbed8edd
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:271:22
        //    9:        0x10607ed08 - std::panicking::default_hook::h6aeaf6d5284e4708
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:290:9
        //   10:        0x10607f59b - std::panicking::rust_panic_with_hook::h1df136699cf9248d
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:692:13
        //   11:        0x10607f4b4 - std::panicking::begin_panic_handler::{{closure}}::h2c310b4a02e33c0f
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:583:13
        //   12:        0x10607e029 - std::sys_common::backtrace::__rust_end_short_backtrace::h62615855b906167c
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/sys_common/backtrace.rs:137:18
        //   13:        0x10607f17d - rust_begin_unwind
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:579:5
        //   14:        0x10609dbf3 - core::panicking::panic_fmt::h81b0ae56717c49a9
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/panicking.rs:64:14
        //   15:        0x10609dd46 - core::panicking::panic_bounds_check::h0570c1ed1da340e8
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/panicking.rs:159:5
        //   16:        0x10605eaf6 - <usize as core::slice::index::SliceIndex<[T]>>::index::ha919b4532a5e772c
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/slice/index.rs:260:10
        //   17:        0x10605e8d6 - core::slice::index::<impl core::ops::index::Index<I> for [T]>::index::hde64a96e0993fef3
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/slice/index.rs:18:9
        //   18:        0x10605e8d6 - <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index::hb6796975a4cb1552
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/alloc/src/vec/mod.rs:2703:9
        //   19:        0x10605be46 - rust_fundamentals::unrecoverable_error::h9f1531aecc4b7dcb
        //                                at /Users/marcellodesales/dev/github.com/marcellodesales/rust-fundamentals/src/main.rs:736:20
        //   20:        0x10605bf39 - rust_fundamentals::main::h0d9d1e890ed67206
        //                                at /Users/marcellodesales/dev/github.com/marcellodesales/rust-fundamentals/src/main.rs:774:5
        //   21:        0x1060541ee - core::ops::function::FnOnce::call_once::h3c7f52b69edf803f
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/ops/function.rs:250:5
        //   22:        0x10605e981 - std::sys_common::backtrace::__rust_begin_short_backtrace::h296e4579310ffcc6
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/sys_common/backtrace.rs:121:18
        //   23:        0x106056ac4 - std::rt::lang_start::{{closure}}::h25e8adc3cc69b121
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/rt.rs:166:18
        //   24:        0x106079200 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h21844b05036a7397
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/core/src/ops/function.rs:287:13
        //   25:        0x106079200 - std::panicking::try::do_call::ha905ecad7031c999
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:487:40
        //   26:        0x106079200 - std::panicking::try::h908fa9e0792f0a82
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:451:19
        //   27:        0x106079200 - std::panic::catch_unwind::h596d02efe9e6f81d
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panic.rs:140:14
        //   28:        0x106079200 - std::rt::lang_start_internal::{{closure}}::he5f89bb7ee56e086
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/rt.rs:148:48
        //   29:        0x106079200 - std::panicking::try::do_call::h42d4b7699af56282
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:487:40
        //   30:        0x106079200 - std::panicking::try::hcf757b5064a6477a
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panicking.rs:451:19
        //   31:        0x106079200 - std::panic::catch_unwind::h7912a5932b004968
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/panic.rs:140:14
        //   32:        0x106079200 - std::rt::lang_start_internal::h4f6d8f6e580c2d8d
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/rt.rs:148:20
        //   33:        0x106056a97 - std::rt::lang_start::hfc9299bb388521bd
        //                                at /rustc/246eae2fab54a5139365c4e1a08c5724fb385fbf/library/std/src/rt.rs:165:17
        //   34:        0x10605bf58 - _main
        //   35:     0x7ff80b6bb310 - <unknown>
    }

    pub fn handle_error_recoverable_errors() {
        // Rust Prelude is the basics of Rust to get it working
        let file_name = "/tmp/data.json";
        open_file(file_name);
        // Os {
        //     code: 2,
        //     kind: NotFound,
        //     message: "No such file or directory",
        // }
        match write_to_file_with_propagation(file_name) {
            Ok(_) => {
                println!("Wrote to file {}", file_name);
            }
            Err(error) => {
                println!("Couldn't write to file {}", file_name);
                println!("{:#?}", error);
            }
        }

        // Now that we have provided propagation of errors, we can call the method with match!
        match read_file_with_propagation(file_name) {
            // Return the match of the result
            Ok(data) => {
                println!("From file: '{}'", data);
            }
            Err(error) => {
                println!("{:#?}", error);
            }
        }
    }

    fn open_file(file_name: &str) {
        match File::open(file_name) {
            Ok(file) => {
                println!("{:#?}", file);
            }
            Err(error) => {
                // Since the file might not exist, then we need to handle the error
                println!("{:#?}", error);
                match error.kind() {
                    ErrorKind::NotFound => {
                        match File::create(file_name) {
                            Ok(file) => {
                                println!("Created file {}", file_name);
                            }
                            Err(error) => {
                                println!("Couldn't create file at {}. Missing permission?", file_name);
                                println!("{:#?}", error);
                            }
                        }
                    }
                    _ => {
                        // as we don't have other options
                        println!("{:#?}", error);
                    }
                }
            }
        }
    }

    fn write_to_file_with_propagation(filename: &str) -> Result<(), Error> {
        // https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x/66484174#66484174
        // the ? after the method call is to abort the method execution and throw the error from here
        let mut file_handler = OpenOptions::new()
            .append(true)
            .open(filename)?;

        // as the method can be error, let's just add the ? into the method
        file_handler.write_all(b"Marcello is here!")?;

        Ok(())
    }

    fn read_file_with_propagation(filename: &str) -> Result<String, Error> {
        // the ? after the method call is to abort the method execution and throw the error from here
        let mut file_handler = File::open(filename)?;
        let mut file_data = String::new();

        // as the method can be error, let's just add the ? into the method
        file_handler.read_to_string(&mut file_data)?;

        // return the 0k only as the handling can be done in other higher-order functions
        Ok(file_data)
    }
}