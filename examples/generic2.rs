#![allow(unused)]

use std::collections::HashMap;

// fn longest_str<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
//     if x.len() > y.len() { x } else { y }
// }
// fn main() {
//     let x = "Hello".to_string();
//     let z = {
//         let y = "Hello Rust".to_string();
//         longest_str(&x, &y)
//     };
// }

// iterator adaptor
fn main() {
    let x = vec![1, 2, 3, 4, 5];

    let y: Vec<i32> = x.iter().map(|f| 2 * f).collect();
    let z: HashMap<String, i32> = x.iter().map(|f| (f.to_string(), f * 2)).collect();

    // filter
    let m: Vec<i32> = x.iter().filter(|f| **f >= 3).map(|d| *d).collect();
    println!("{:?}", m);
}
