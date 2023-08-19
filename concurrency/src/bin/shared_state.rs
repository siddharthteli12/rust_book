use std::sync::Mutex;

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
}
