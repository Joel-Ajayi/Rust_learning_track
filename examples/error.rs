#![allow(unused)]

// fn main() {
//     let y: Option<i32> = None;

//     match y {
//         Some(val) => println!("{:?}", val),
//         None => println!("None"),
//     }

//     let result: Result<i32, String> = if y != None {
//         Ok(200)
//     } else {
//         Err("Error Here".to_string())
//     };

//     match result {
//         Ok(val) => println!("{:?}", val),
//         Err(val) => println!("{:?}", val),
//     }
// }

// panic!. e.g panic!("Something went wrong");
// Option<type>
// Result<type1, type1>

fn main() {
    let y: Option<i32> = None;

    match y {
        Some(val) => println!("{:?}", val),
        None => println!("None"),
    }

    let result: Result<i32, String> = if y != None {
        Ok(200)
    } else {
        Err("Error Here".to_string())
    };

    match result {
        Ok(val) => println!("{:?}", val),
        Err(val) => println!("{:?}", val),
    }

    // variable.unwrap() or var.expect(ms) and returns vals of option
    //  or result but panics if the values are "None" or "Err("..")""
    let x: i32 = "yo".to_string().parse().expect("Invalid Number");

    println!("The val is {x}");
}
