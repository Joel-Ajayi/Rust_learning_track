#![allow(unused)]

use std::cmp::PartialOrd;
use std::cmp::min;

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Traits
struct Solidity {
    version: String,
}

struct Vyper {
    version: String,
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {}", file_path)
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {}", file_path)
    }
}

impl Solidity {
    fn new(version: String) -> Self {
        Self { version: version }
    }
}

trait List<T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

impl<T> List<T> for Vec<T> {
    fn count(&self) -> usize {
        self.len()
    }
    fn first(&self) -> &T {
        &self[0]
    }
}

// fn main() {
//     let sol = Solidity::new("0.9".to_string());
//     let file_path: &str = "howfa";
//     let compile_path = sol.compile(file_path);

//     let arr: Vec<u32> = vec![5; 5];

//     println!(
//         "array count is {} and first val is {}",
//         arr.count(),
//         arr.first()
//     );
// }

// Use PartialOrd to confirm types can be compared
// Henece type "T" must implement PartialOrd
fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x <= y { x } else { y }
}

trait A {}
trait B {}
trait C {}
impl A for u32 {}
impl B for u32 {}
impl C for i32 {}

// to use func, the type T have to implement A and B
fn w<T: A + B, U: B + C>(x: T, y: U) {}

pub fn zip<T: Copy, U: Copy>(a: Vec<T>, b: Vec<U>) -> Vec<(T, U)> {
    let mut v = vec![];
    let len = min(a.len(), b.len());
    for i in 0..len {
        v.push((a[i], b[i]));
    }
    v
}

fn main() {
    let arr: Vec<u32> = vec![3; 5];
    let arr2: Vec<i32> = vec![-5; 5];

    let res = zip(arr, arr2);
    println!("{:?}", res);
}

// Each input reference parameter gets its own lifetime parameter.
// If there is exactly one input lifetime parameter, that lifetime
//      is assigned to all output lifetime parameters.
// If there are multiple input lifetime parameters,
// but one of them is &self or &mut self,
//      the lifetime of self is assigned to all elided output lifetime parameters.
