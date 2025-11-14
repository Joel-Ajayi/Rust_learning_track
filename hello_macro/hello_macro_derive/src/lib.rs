// syn: This is the "parser" library. It's the "robot's eyes."
// It lets you parse the raw Rust code into a tree structure you can analyze.

// quote: This is the "generator" library. It's the "robot's hands."
// It lets you easily write new Rust code.

// `proc_macro` is built into Rust and gives us the entry point.
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let generator = quote! {
        impl hello_macro::HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    generator.into()
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // The compiler gives us a `TokenStream`, which is just text.
    // We use `syn` to *parse* this text into a "syntax tree" (ast)
    // that we can actually understand.
    let ast = parse_macro_input!(input as DeriveInput);

    impl_hello_macro(&ast)
}
