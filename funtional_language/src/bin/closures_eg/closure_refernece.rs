pub fn reference_ownership() {
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
}
