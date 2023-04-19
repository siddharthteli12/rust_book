use std::io::{Error, ErrorKind};

fn main() {
    // Convert result type to option with ok.
    match returns_result(12).ok() {
        Some(value) => println!("Value returned {:}", value),
        None => println!("None"),
    };

    match returns_result(5).ok() {
        Some(value) => println!("Value returned {:}", value),
        None => println!("None"),
    };

    // Convert option type to result with ok_or.
    match returns_option(12).ok_or(Error::new(ErrorKind::Other, "Custom error")) {
        Ok(value) => println!("Value returned {:}", value),
        Err(e) => println!("Error -- {:}", e),
    };

    match returns_option(5).ok_or(Error::new(ErrorKind::Other, "Custom error")) {
        Ok(value) => println!("Value returned {:}", value),
        Err(e) => println!("Error -- {:}", e),
    };
}

fn returns_result(value: i32) -> Result<i32, Error> {
    if value > 10 {
        Ok(value)
    } else {
        Err(Error::new(ErrorKind::Other, "Custom error"))
    }
}

fn returns_option(value: i32) -> Option<i32> {
    if value > 10 {
        Some(value)
    } else {
        None
    }
}
