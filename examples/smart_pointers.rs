// smart pointers
// Memory Management: Smart pointers like Box<T> allow for heap allocation,
// which is essential for data whose size is not known at compile time
// or for transferring ownership of large data structures without copying.
// Box<T>: mutable and immutable borrowing checked at compile time
// Rc<T>: multiple immutable owners of data
// Arc<T>:thread-safe shared ownership
// RefCell<T>: mutate data even when you only have an immutable reference to it because of check at runtime
// Weak<T>: break reference cycles and prevent memory leaks in cyclic data structures.

#[warn(unused_imports)]

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    let list = Cons(1, Box::new(Nil));
}
