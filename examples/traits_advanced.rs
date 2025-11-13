// **1 Associated types and Generic type
// Once concrete type per implementation
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

// for generics, multiple types
struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}
// *conflicting implementations of trait `Iterator` for type `Counter`
// we can only have type u32 as it is
// impl Iterator for Counter {
//     type Item = u16;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(0)
//     }
// }

pub trait IteratorGeneric<T> {
    fn next(&mut self) -> Option<T>;
}

impl IteratorGeneric<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

impl IteratorGeneric<u16> for Counter {
    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}

fn main1() {}

// **2 Default Generic Type Param
use std::{fmt::Debug, ops::Add};

use futures::future::Select;
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// trait Add<Rhs = Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main2() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

// **3 Methods with same name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main3() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // for specific traits
    // <Human as Wizard>::fly();
}

// **4 Using Supertraits
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point2 {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point2 {}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/*orphan rule
We can define a trait on a type as long as the
 trait or type is defined in our crate.
 Example here is Display trait, not defined on our crate, but Vec
 type is also not defined in our crate.
 Solution
*/
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let p = Point2 { x: 3, y: 3 };

    p.outline_print();

    //     let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    //     println!("w = {w}");
}
