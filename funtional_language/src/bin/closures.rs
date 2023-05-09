use std::{thread, time::Duration};

// Testing closures.
fn main() {
    let test_closure = |name| {
        thread::sleep(Duration::new(2, 0));
        println!("Hello my name is {:}", name);
        name
    };

    test_closure(String::from("Siddharth"));
}
