#![allow(unused)]

// use std::collections::
// Library Crates:
// These crates compile into a library that can be used by other programs or crates.
// They typically have a lib.rs file, which is the root of the library's module tree.

// Module rules
// 1. A package must have at least one crate
// 2. A package can have 0 or 1 library create
// 3. A // can have any number of binary creates
mod house_front;

fn serve_order() {}

mod house_back;

use house_back::Brakefast;

pub fn eat_at_res() {
    let mut meal = Brakefast::summer("toast");
    meal.toast = "Orange".to_string();
}

pub fn add_two(val: i32) -> i32 {
    2 + val
}

mod tests {
    #[test]
    fn check_eql() {
        assert_eq!(2 + 2, 4, "Tried to addd two values {} and {}", 2, 2);
    }

    #[test]
    // with exact panic message for debugging
    #[should_panic(expected = "reason")]
    fn check_neeql() {
        panic!("reason");
    }
}
