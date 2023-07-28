// compiler seach file1 in current level.
mod file1;
use file1::{
    check_div,
    maths::{add, sub},
};

// When we use mod there should be folder.rs file or mod.rs inside folder.
mod folder;
use folder::{test1, test2};
fn main() {
    let x = 12;
    let y = 15;

    println!("Is x divisible by y? - {:}", check_div(&x, &y));
    println!("x + y {:}", add(&x, &y));
    println!("x - y {:}", sub(&x, &y));
    test1();
    test2();
}
