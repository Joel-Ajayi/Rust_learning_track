// **1 Using the Newtype Pattern for Type Safety and Abstraction

fn main2() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        let f: Thunk = Box::new(|| println!("hi"));
        f
    }
}

// Dynamically Sized Types and the Sized Trait

// T may be sized or not: dynamic
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}

fn main() {
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";
}
