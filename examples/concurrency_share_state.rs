use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
// Arc<T>:thread-safe shared ownership. Rc is not thread safe so we use Arc

fn main() {
    // let m = Mutex::new(5);

    // {
    //     // we have to clock it before be we mutate it
    //     let mut num = m.lock().unwrap();
    //     *num = 6;

    //     // unlocking is done automatically
    // }

    // println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counterClone = counter.clone();
        let handle = thread::spawn(move || {
            // Mutex has internal mutability like Refcell
            let mut num = counterClone.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result {}", *counter.lock().unwrap())
}
