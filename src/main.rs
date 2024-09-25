mod my_func;
mod other_func;

// use crate::my_func::{add_five,subtract_five};
// use crate::my_func::*;

use crate::my_func::add_five;
use crate::other_func::subtract::subtract_five;

fn main() {
    let mut x: u32 = 50;
    println!("x value{x}");

    let y: u32 = add_five(x);
    println!("y value {y}");

    let z: u32 = subtract_five(x);
    println!("z value {z}");

    x = 70;
    println!("new x value {x}");
}
