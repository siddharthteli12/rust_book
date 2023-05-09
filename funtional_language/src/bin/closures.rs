use std::{thread, time::Duration};

fn main() {
    let test_closure = |name: String| -> String {
        thread::sleep(Duration::new(2, 0));
        println!("Hello my name is {:}", name);
        name
    };

    test_closure(String::from("Siddharth"));
}
