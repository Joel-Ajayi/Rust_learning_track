// use std::collections::HashMap;
#[allow(unused)]

// fn main() {
// signed integers
// in: -(2^(n-1)) to 2^(n-1) -1

// unsigned intergers(non negetaive)
// un: 0 to 2^(n-1) -1
// unsigned integers

//     // let mut howfa = HashMap::new();

//     // howfa.entry("name").or_insert("Ayodeji");
//     // howfa.insert("age", "32");
//     // println!("{:?}", howfa.get("name"));

//     let arr = vec![1, 3, 4, 5, 8];

//     for i in 0..arr.len() {
//         let resut = if arr[i] == 4 {
//             "Howfa mehn"
//         } else {
//             "Howfa now"
//         };

//         match i {
//             1 | 3 => println!("Match 1 or 3: i is {}, but {}", arr[i], resut),
//             4 | 5 => println!("Match 4 or 5: i is {}, but {}", arr[i], resut),
//             _ => println!("Match others: i is {}, but {}", arr[i], resut),
//         }
//     }
// }

// fn main() {
//     let x: Option<i32> = Some(12);

//     match x {
//         Some(val) => print!("Option is {val}"),
//         None => {}
//     }

//     let y: String = "Hey".to_string();

//     // shorter code
//     if let Some(val) = x {
//         print!("\n Option is {val}")
//     }
// }

fn main() {
    // strings require .copy to move. numbers, chars and booleans do not.
    // Stack - at compile time are stored - fixed varibales - step by step
    // Heap - at run time are stored that can grow or shrink
    // Heap requires an allocator to fiind address(There is a gradeoff b/w time and heap frag)
    // heap frag
    let x = "Hey".to_string();
    let y = &x;

    let a = 5;
    let b = a;

    println!("a = {}", a);
    println!("b = {}", b);

    println!("x = {x}");
    println!("y = {y}");
}
