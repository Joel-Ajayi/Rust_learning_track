// **1 more than one mutable reference
use std::slice;
fn main1() {
    // A raw pointer (*const i32 or *mut i32)
    let mut num = 5;

    // raw pointer
    // The compiler gives you ZERO guarantees. A raw pointer could be:
    // Null.
    // Dangling (pointing to freed memory).

    // Pointing to the wrong type of data.
    // we cannot typicall have a mutable and unmutable borrowing
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is:{}", *r1);
        println!("r2 is:{}", *r2);
    }

    // Unsafe functions must be called in an unsafe block
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    let mut v = vec![2, 4, 5];
    let r = &mut v[..];

    r.split_at_mut(1);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // violates the only one borrowed mutable ref rule at a time
    unsafe {
        // (&mut slice[..mid], &mut slice[mid..])
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// ***2 FFI
// external code in diff language FFI(Foreign language Interface)
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main2() {
    unsafe {
        println!("Absolute of -4 according to C: {}", abs(-4));
    }
}

// other language can call our rust prgram
// mangle:when compiler chnages name of a func for more info to other part of compilation
// prevent mangling
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("just called a Rust func from C!");
}

// **3
// modify mutable static varibales
// global varibales are static variables
// they must have static lifetime
// they must be annotated(&str, u32, etc)
// *Diff with Const varibale
// they have fixed address(no duplication)
// they can be mutable but it is unsafe
static mut COUNTER: u32 = 0;

// make the operation explicitly unsafe to call
unsafe fn add_to_count(inc: u32) {
    COUNTER += inc;
}

fn main() {
    // call the unsafe function and access the static inside an unsafe block
    unsafe {
        add_to_count(3);
        // copy the value out of the mutable static to avoid creating a shared reference to it
        let counter_val = COUNTER;
        println!("Counter {}", counter_val);
    }
}
