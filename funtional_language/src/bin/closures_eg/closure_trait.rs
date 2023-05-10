pub fn fn_trait() {
    // Condition 1- Closure impl Fnonce trait which means it can only be called once.
    // Beause closure move the parent variable. Hence, calling it again variable would be invalid.
    let vec1 = vec![1, 2, 3, 4];
    // test1 impl FnOnce trait.
    let test1 = || vec1;
    test1();
    //test1(); Error - Closure cannot be invoked more than once because it moves value name.

    // Condition 2- Closure impl FnMut trait, It can be called multiple times.
    // Closure doesn't move external value but mut it.
    let mut vec3 = vec![1, 2, 3, 4];
    // test1 impl FnMut trait.
    let mut test1 = || vec3.push(5);
    test1();
    test1(); // Can be called multiple times.

    // Condition 3- Closure impl Fn trait, can be called multiple times.
    // Closure doesn't move external value or mut it.
    let vec3 = vec![1, 2, 3, 4];
    // test1 impl Fn trait.
    let test1 = || println!("Value of vec - {:?}", vec3);
    test1();
    test1(); // Can be called multiple times.
}
