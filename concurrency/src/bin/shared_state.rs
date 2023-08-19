#![allow(unused_variables)]
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug)]
struct NuclearCode(i32);
fn main() {
    // Create mutex.
    let value = Mutex::new(NuclearCode(100));

    // MutexGuard is a smart pointer.
    if let Ok(mut value) = value.lock() {
        // It impls Deref trait. Hence, *value returns wrapped data.
        *value = NuclearCode(100);
    }
    // Confirm value is updated.
    println!("Nuclear code - {:?}", value);

    mutex_with_multiple_threads();
    mutex_with_rc_with_multiple_threads();
    mutex_with_arc_with_multiple_thread();
}

// Eg to test mutex with multiple threads.
fn mutex_with_multiple_threads() {
    let value = Mutex::new(100);
    for _ in 0..10 {
        // Error, because value is moved here because of move.
        // Even though lock is only taking ref of value
        // We have to use move because this thread can outlive function body.

        // thread::spawn(move || {
        //     let mut nuclear_code = value.lock().unwrap();
        //     *nuclear_code = *nuclear_code + 1;
        // });
    }
}

// Eg to test mutex with Rc smart pointer.
fn mutex_with_rc_with_multiple_threads() {
    let value = Rc::new(Mutex::new(100));

    for _ in 0..10 {
        let thread_value = value.clone();

        // Even rc smart pointer doesn't work here.
        // Because rc are restricted to single thread.

        // thread::spawn(move || {
        // let mut local_value = thread_value.lock().unwrap();
        // *local_value = *local_value + 1;
        // });
    }
}

// Eg to test mutex with Arc smart pointer.
fn mutex_with_arc_with_multiple_thread() {
    let value = Arc::new(Mutex::new(100));
    let mut thread_handler = vec![];
    for _ in 0..10 {
        let thread_value = value.clone();
        let thread = thread::spawn(move || {
            let mut local_value = thread_value.lock().unwrap();
            *local_value = *local_value + 1;
        });

        thread_handler.push(thread);
    }
    // Finish all threads
    for handle in thread_handler {
        handle.join().unwrap();
    }

    println!("Final value - {:?}", value);
}
