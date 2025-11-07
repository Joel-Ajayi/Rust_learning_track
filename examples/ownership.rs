#![allow(unused)]

fn take(s: &mut String) {
    s.clear();
    s.push_str("Charlie howfa");
    println!("Taken: {s}");
}

// rules
// 1. Ref borrows and does not take ownserhip
// 2. You cannot have both mutable and non-mutable in same scope
// 3. A ref cannot out live the owner

fn main() {
    let mut s: String = "Howfa chief".to_string();

    take(&mut s);

    println!("{s}");
}
