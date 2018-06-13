extern crate clib1_sys;
use clib1_sys::{square2};


fn main() {
    let r = square2(3);
    println!("3 squared is {}", r);
}
