fn main() {
    println!("Hello, world!");
    unsafe {
        access_pointer();
    }
}

// This function is accessing raw pointer.
unsafe fn access_pointer() {
    let mut value = 1;
    let pointer = &mut value as *mut i32;
    println!("Value of pointer {:?}", *pointer.add(1));
}
