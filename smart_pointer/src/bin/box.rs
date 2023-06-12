#![allow(dead_code)]

use std::ops::Deref;
fn main() {
    simple_box_eg();
    resursive_type_eg();
}

// Simple smart pointer eg.
fn simple_box_eg() {
    // i32 wrapped in box.
    let box_val = Box::new(50);
    // Simple i32.
    let val = 50;
    // Box wrapped type are treated normally & act like a ref.
    assert_eq!(val, *box_val);
}

// Box use case 1.
// Resursive type.
#[derive(Debug)]
enum List<T> {
    VALUE { val: T, next: Box<List<T>> },
    None,
}

fn resursive_type_eg() {
    let list1 = List::VALUE {
        val: 12,
        next: Box::new(List::VALUE {
            val: 13,
            next: Box::new(List::None),
        }),
    };

    println!("List value - {:?}", list1);
}

// Define custom smart pointer type.

#[derive(Debug)]
struct CustomBox<T>(T);

impl<T> CustomBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for CustomBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn custom_box_eg() {
    let custom_box = CustomBox::new(23);
    // Compiler converts *T to *(T.deref()).
    assert_eq!(23, *custom_box);
    println!("Sid box value - {:?}", custom_box);
}
