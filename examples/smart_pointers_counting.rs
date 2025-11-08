// Memory Management: Smart pointers like Box<T> allow for heap allocation,
// which is essential for data whose size is not known at compile time
// or for transferring ownership of large data structures without copying.
// Rc<T>: multiple immutable owners of data
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));
    // borrows value
    let a = Rc::new(Cons(value.clone(), Rc::new(Nil)));
    // 2nd ownser of a
    let b: List = Cons(Rc::new(RefCell::new(3)), a.clone());
    // 3rd ownser of a
    let c: List = Cons(Rc::new(RefCell::new(5)), a.clone());

    *value.borrow_mut() += 10;

    // print ref count
    println!("Count is {}", Rc::strong_count(&value));
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
