#![allow(unused)]

// Binary Crates:
// These crates compile into an executable program.
// They typically have a main.rs file containing
// the main function, which serves as the entry point for the program.
#[derive(Debug, PartialEq)]
enum Command {
    Play,
    Stop,
    Back(u32),
    Skip(u32),
    Resize { width: u32, height: u32 },
}

// fn main() {
//     // let x: String = String::from("Howfa");
//     // let y: &str = &x[..2];

//     // println!("{y}");
//     // println!("{x}");

//     let cmd: Command = Command::Play;
//     let cmd2: Command = Command::Skip(32);
//     print!("{:?}", cmd2);

//     // Option<T> = Some(T) | None
//     let x: Option<i32> = Some(1);
//     print!("\n {:?}", x);

//     // Result<T, E> = Ok(T) | Err(E)
//     let x: Result<i32, String> = Ok(100);
//     let x: Result<i32, String> = Err("Error there".to_string());
//     print!("\n {:?}", x);
// }

#[derive(Debug)]
struct Person {
    name: String,
    ag: u32,
}

// fn main() {
//     let user = Person {
//         name: "Ay".to_string(),
//         ag: 32,
//     };

//     print!("{:?}", user);

//     print!("{}", user.name);
// }

fn main() {
    // v.pop() .push(val), insert(index, val), .remove(index), .len()
    //
    let mut v = vec![20i32; 20];

    print!("{:?} \n", v);

    let mut v: Vec<i32> = Vec::new();
    v.push(3);

    print!("{:?}", v);

    // For out of bound case
    print!("\n{:?}", v.get(100)); // Option<&i32> = Some(&i32) | None
}
