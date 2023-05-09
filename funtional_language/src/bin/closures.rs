use std::{thread, time::Duration};

fn main() {
    let test_closure = |name| {
        thread::sleep(Duration::new(2, 0));
        println!("Hello my name is {:}", name);
        name
    };

    test_closure(String::from("Siddharth"));
    closure_ref_ownership_test();
}

fn closure_ref_ownership_test() {
    // Condition 1- Closure takes immutable ref.
    let vec1 = vec![1, 2, 3, 4];

    let closure1 = || println!("Value of vec1 {:?}", vec1);
    println!("Value of vec1 {:?}", vec1);
    closure1();
    println!("Value of vec1 {:?}", vec1);

    // Condition 2- Closure takes mutable ref.
    let mut vec2 = vec![1, 2, 3, 4];
    // Closure appends something to vec.
    // Hence, taking mutable ref implicitly.
    let mut closure1 = || vec2.push(10);
    // println!("Value of vec1 {:?}", vec2); Error - Cannot borrow.
    closure1();
    println!("Value of vec2 {:?}", vec2);

    // Condition 3- Closure takes ownership.
    let mut vec3 = vec![1, 2, 3, 4];
    // move takes explicit ownership of variable used in closure.
    let mut closure1 = move || vec3.push(10);
    //println!("Value of vec3 {:?}", vec3); Error - Borrow of moved value.
    closure1();

    // Condition 4- Closure inside new thread takes ownership.
    let vec4 = vec![1, 2, 3, 4];
    // move is mandatory coz ref value can become invalid if parent thread where value is declared completes first.
    thread::spawn(move || {
        println!("Value of vec4 {:?}", vec4);
    })
    .join()
    .unwrap();
}
