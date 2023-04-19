use std::{fs::File, io::Read};

// ? is used to propogate error.
fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut file = File::open("user.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    // Error handling at caller.
    match read_username_from_file() {
        Ok(info) => println!("Username from file-- {:}", info),
        Err(e) => println!("Unable to read file sorry, reason - {:}", e),
    }
}
