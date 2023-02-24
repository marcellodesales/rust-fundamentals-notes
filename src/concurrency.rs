/**
 * * Go rust-lang.org/community
 * * Many threads receipes at https://rust-lang-nursery.github.io/rust-cookbook/concurrency.html
 * * All chapter https://doc.rust-lang.org/book/ch16-00-concurrency.html
 * *
 */
pub mod Concurrency {

    use std::thread;
    use std::sync::mpsc;
    use std::sync::mpsc::{Sender, Receiver};

    pub fn create_thread() {
        let outer_scope = 412;

        println!("Will create a thread join handler!");
        // the thread doesn't own the data in the outer scope, so when it's out of scope,
        // the data that the thread is pointing to is gone. The variable on the thread
        // has outlived the owner. So, the thread has to take ownership of the data if
        // it's going to work properly.
        // In order to make it work, we use the keyword "move" before the closure declaration!
        // Remember since our thread now owns outer_scope, we can't use it in our function that
        // spawned thread anymore.
        let calculator_thread = thread::spawn(move || {
            println!("Calculating value within the thread...");

            // ATTENTION!!! Since this is the last statement of the closure, it is the returned
            // If using ;, it will be a void method call!!! The result option would be empty!
            outer_scope * 2
        });

        // In order to get the value back from the thread, we need to know when the thread
        // is finished! Since it's independently from any other code, will it be even done
        // in time for us to get that data back from it before we need to use it?
        // This is where we get back to that join handler.

        // Blocking operation that will wait for the thread to finish
        let result = calculator_thread.join();
        match result {
            Ok(value) => {
                println!("Calculated result from thread: {:?}", value);
            }
            Err(_) => {
                println!("Error occurred while computing value in thread!");
            }
        }
    }

    pub fn threads_communication_by_messaging_passing() {
        // threads are by their nature independent units of execution with resources like mem and CPU.
        // Even through they are on the same program, it's conceptualy similar to two separate
        // programs running in their processes.

        // Sharing memory data has a lot of dangers associated with it! We get into locks, mutexes,
        // semaphores, which means there's more code needed. It's pretty complex code, so we are
        // going to use message passing through channels!

        // Channels are going in one way only! If Sarah and John are talking, we need 2 channels!
        // Since they are producers of data and single consumers (talking to each other)
        // We are going to use mpsc - multiple producers, single consumers.


        let (john_sender_channel , john_receiver_channel) = mpsc::channel();
        let (sarah_sender_channel, sarah_receiver_channel) = mpsc::channel();

        // We need to spaw threads for each participant!
        let john_thread = thread::spawn(move || {
            johns_chat(sarah_sender_channel, john_receiver_channel);
        });

        let sarah_thread = thread::spawn(move || {
            sarahs_chat(john_sender_channel, sarah_receiver_channel);
        });

        match john_thread.join() {
            Ok(_) => {
                println!("John's thread just finished!")
            }
            Err(_) => {}
        }

        match sarah_thread.join() {
            Ok(_) => {
                println!("Sarah's thread just finished!")
            }
            Err(_) => {}
        }
    }

    fn sarahs_chat(john_sender: Sender<&str>, sarah_receiver: Receiver<&str>) {
        // Receive and Try-Receive methods of the receiver
        // recv will block, not allowing the thread to run any more code
        // try_recv will continue the execution without blocking
        let sarahs_message = sarah_receiver.recv();
        println!("{}", sarahs_message.unwrap());

        // When sarah receives a message, she sends another one to john right after!
        let _send_result = john_sender.send("Hello John... How's it going?");
    }

    fn johns_chat(sarah_sender: Sender<&str>, john_receiver: Receiver<&str>) {
        // John starts the conversation, so he needs to send the first message!
        let _send_result = sarah_sender.send("Hi Sarah! How are you?");

        // John then, waits for sarah's reply! It will be blocked until Sarah's reply!
        let johns_message = john_receiver.recv();
        println!("{}", johns_message.unwrap());
    }
}