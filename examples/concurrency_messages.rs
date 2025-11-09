// Multiple Producer, Single Consumer at compile time
// The Analogy: Instead of one shared whiteboard,
// each worker (thread) gets their own private office with their own private whiteboard.

/*
The How So how do they work together? When Worker A needs Worker B to know something,
    Worker A writes a memo (a message) and sends it through a mail tube
    (a channel) to Worker B's office.

The Rust Superpower When Worker A sends the memo, they give up ownership of it.
    They can't edit it anymore. Worker B receives the memo and now owns it.
    The Rust compiler enforces this at compile time.
*/

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = channel();
    let sender2 = sender.clone();

    thread::spawn(move || {
        // let msg = String::from("hi");

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread"),
        ];

        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // cannot do this because ownership has been tranfered
        // println!("msg is {}", msg);
    });

    thread::spawn(move || {
        // let msg = String::from("hi");

        let vals = vec![
            String::from("hi2"),
            String::from("from2"),
            String::from("thread2"),
        ];

        for val in vals {
            sender2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // cannot do this because ownership has been tranfered
        // println!("msg is {}", msg);
    });

    for received in receiver {
        print!("Got {} \n", received);
    }
}
