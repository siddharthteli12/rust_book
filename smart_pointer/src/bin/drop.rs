#![allow(unused_mut, unused_variables)]

use std::fmt::Debug;

struct CustomStruct<T: Debug>(T);

impl<T: Debug> Drop for CustomStruct<T> {
    fn drop(&mut self) {
        println!("Drop is called on {:?}", self.0);
    }
}

fn main() {
    let custom_value1 = CustomStruct("custom_value1");
    let mut custom_value3 = CustomStruct("custom_value3");
    println!("Before scope");
    {
        let custom_value2 = CustomStruct("custom_value2");
    }
    // Cannot call struct drop method directly.
    // custom_value3.drop();

    // This will invoke structs drop method.
    drop(custom_value3);

    // custom_value3 is dropped now.
    //println!("{:?}", custom_value3);

    println!("After scope");
}
