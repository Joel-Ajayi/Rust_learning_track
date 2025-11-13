#[derive(Debug)]
enum Language {
    English,
    Spanish,
    Russian,
    Japanese,
}

//

fn main() {
    let language = Language::Spanish;

    match language {
        Language::English => println!("Howfa"),
        _ => println!("Jo mapel"),
    }

    // Irrefutable
    let x: i32 = 5;

    // Refutable
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    }

    // Can only accept irrefuatble patterns:
    // function parameters
    // let statements
    // for loops
}
