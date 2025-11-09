use std::{thread, time::Duration};

fn main() {
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawn thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    // block(prevent exit or from doing any further work) the thread currently running(main thread)
    // until the handler thread finishes before main closes
    handler.join().unwrap();

    // Context two. using a
    let v = vec![1, 2, 3];

    // we use the "move" keywared to move ownership to closure
    // That is because since we are using it inside the thread, and
    // they is no direct order operation be main and spawn threa, v may
    // have been dropped when trying to access it in spawn thread
    let handler2 = thread::spawn(move || {
        println!("here is a vector {:?}", v);
    });

    handler2.join().unwrap();
}
