use std::rc::Rc;
use List::*;

#[derive(Debug)]
enum List<T> {
    Val(T, Rc<List<T>>),
    None,
}
// Simple reference counting (RC) eg.
fn main() {
    // Simple recursive list.
    let a = Rc::new(Val(30, Rc::new(Val(40, Rc::new(None)))));

    // Both b & c share a.
    let b = Val(10, Rc::clone(&a));
    println!("a rc count - {:}", Rc::strong_count(&a));
    let c = Val(20, Rc::clone(&a));
    println!("a rc count - {:}", Rc::strong_count(&a));

    println!("Value of b - {:?}", b);
    println!("Value of b - {:?}", c);
}
