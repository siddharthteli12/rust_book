#![allow(dead_code)]
fn main() {
    simple_box_eg();
    resursive_type_eg();
}

// Simple smart pointer eg.
fn simple_box_eg() {
    // String wrapped in box.
    let box_sur = Box::new("Teli".to_string());
    // Simple string.
    let mut first_name = String::from("Siddharth");
    // Box wrapped type are treated normally.
    first_name.push_str(&box_sur);
    // Appending String with Box<String> works fine.
    println!("Value of ref to box - {:}", first_name);
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
