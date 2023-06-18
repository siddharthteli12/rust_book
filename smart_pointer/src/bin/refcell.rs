use std::cell::RefCell;

fn main() {
    // ref_eg is immutable.
    let ref_eg = RefCell::new(vec![1]);
    // But still its value is changed because 
    // its defined inside refcell which allows to mutate imutable types.
    ref_eg.borrow_mut().push(2);

    println!("Value of ref_eg- {:?}", ref_eg);
}
