use std::{thread, time::Duration};

pub fn thread1() {
    // Condition 1- Simple closure with thread sleep.
    let test_closure = |name| {
        thread::sleep(Duration::new(2, 0));
        println!("Hello my name is {:}", name);
        name
    };

    test_closure(String::from("Siddharth"));

    // Condition2- Closure inside new thread takes ownership.
    let vec2 = vec![1, 2, 3, 4];
    // move is mandatory coz ref value can become invalid if parent thread where value is declared completes first.
    thread::spawn(move || {
        println!("Value of vec2 {:?}", vec2);
    })
    .join()
    .unwrap();
}
