// Macros allow you write code to help you write other codes
// It is know as meta programming
// Like a function where input is code and ouput is also code
// functions are called at runtime and macros are expanded before code finish compilation

// **1 Declarative Macros
// vec macro
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// **2 Procedural Macro
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
