// Simple smart pointer eg.
fn main() {
    // String wrapped in box.
    let box_sur = Box::new("Teli".to_string());
    // Simple string.
    let mut first_name = String::from("Siddharth");
    // Box wrapped type are treated normally.
    first_name.push_str(&box_sur);
    // Appending String with Box<String> works fine.
    println!("Value of ref to box - {:}", first_name);
}
