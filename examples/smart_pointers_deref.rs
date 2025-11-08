// smart pointers
// Memory Management: Smart pointers like Box<T> allow for heap allocation,
// which is essential for data whose size is not known at compile time
// or for transferring ownership of large data structures without copying.
// Rc<T>: multiple immutable owners of data
// Arc<T>:thread-safe shared ownership
// Weak<T>: break reference cycles and prevent memory leaks in cyclic data structures.
// RefCell<T>: mutate data even when you only have an immutable reference to it
use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Implicit Deref Coesion is a func that converts
// a ref of one type to a ref of another type
// It does this for:
// a. &T to &U when T:Deref<Target=U>
// b. &mut T to &mut U when T:DefrefMut<Target=U>
// c &mut T to &U when T:Deref<Target=U>

fn main() {
    let x: i32 = 5;
    // a copy of 5.
    let mut y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *(y.deref_mut()));

    // Deref Coesion
    let m = MyBox::new(String::from("Rust"));
    // &MyBox<String> -> &String -> &str
    // Rust has auto coesion
    hello(&m);
}

fn hello(name: &str) {
    print!("Hello, {}!", name)
}
