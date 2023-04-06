fn main() {
    test_move();
}

// Test the concept of move.
fn test_move() {
    // Value is not moved for prmitive type.
    // Rather new value 100 is stored on stake & y points to it.
    let x: i32 = 100;
    let y = x;
    println!("Is X valid - {:?}", x);

    // Same doesn't happen with string type.
    // value is moved, ownership is transferred.
    let string_1 = String::from("Siddharth");
    let string_2 = string_1;
    // Error - value borrowed here after move
    // println!("Is string_1 valid - {:?}", string_1);
}
