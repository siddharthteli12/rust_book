#![allow(dead_code)]
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
