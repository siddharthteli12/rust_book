use std::{fs::File, io::Read};

#[derive(Debug)]
struct SidError(std::io::Error);

// Convert from Io error to custom error.
impl From<std::io::Error> for SidError {
    fn from(value: std::io::Error) -> Self {
        Self(value)
    }
}
// ? converts error type from source to current function error type.
// For that it finds from method impl from source error type to current function error type.
fn read_username_from_file() -> Result<String, SidError> {
    // File open will return io error type but this function has SidError type.
    // Hence, convertion will happen through from impl above.
    let mut file = File::open("user.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    // Error handling at caller.
    match read_username_from_file() {
        Ok(info) => println!("Username from file-- {:}", info),
        Err(e) => println!("Unable to read file sorry, reason - {:?}", e),
    }
}
