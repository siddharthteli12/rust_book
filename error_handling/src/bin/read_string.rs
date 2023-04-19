use std::fs;
use std::io::Error;
fn main() {
    match read_string_from_file() {
        Ok(info) => println!("Success, String -- {:}", info),
        Err(e) => println!("Error, reason -- {:}", e),
    }
}

fn read_string_from_file() -> Result<String, Error> {
    fs::read_to_string("user.txt")
}
