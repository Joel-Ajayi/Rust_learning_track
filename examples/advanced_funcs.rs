fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

fn main1() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}

fn main2() {
    let list_of_numbers = vec![1, 2, 3];
    // with named function
    // let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    // with closures
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
}

fn main() {
    fn returns_closure(a: i32) -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    // for more than one closure
    // each closure is a unique type
    fn return_optional_closures(a: i32) -> Box<dyn Fn(i32) -> i32> {
        if a > 0 {
            Box::new(move |b| a + b)
        } else {
            Box::new(move |b| a - b)
        }
    }
}
