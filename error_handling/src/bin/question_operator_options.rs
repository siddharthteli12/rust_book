use std::fs;
use std::io::Error;
fn main() {
    match read_string_from_file() {
        Ok(info) => {
            let last_char_result = return_last_char_from_string(&info);
            match last_char_result {
                Some(last_char) => println!("Success, Last char -- {:}", last_char),
                None => println!("Got none"),
            };
            println!("Success, String -- {:}", info);
        }
        Err(e) => println!("Error, reason -- {:}", e),
    }
}

fn read_string_from_file() -> Result<String, Error> {
    fs::read_to_string("user.txt")
}

// ? Can be used on option too.
// Returns early if none.
fn return_last_char_from_string(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
