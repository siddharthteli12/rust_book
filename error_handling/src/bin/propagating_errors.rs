use std::{fs::File, io::Read};

// This funtion doesn't panic but returns result type so error can be handled at caller funtion.
fn read_username_from_file() -> Result<String, std::io::Error> {
    match File::open("user.txt") {
        Ok(mut file) => {
            let mut username = String::new();
            match file.read_to_string(&mut username) {
                Ok(_) => Ok(username),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

fn main() {
    // Error handling at caller.
    match read_username_from_file() {
        Ok(info) => println!("Username from file-- {:}", info),
        Err(e) => println!("Unable to read file sorry, reason - {:}", e),
    }
}
