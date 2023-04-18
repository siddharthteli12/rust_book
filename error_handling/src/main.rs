use std::{fs::File, io::ErrorKind};

fn main() {
    let test_file = match File::open("test.txt") {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating new file {:?}", e),
            },
            other_error => {
                panic!("Problem creating new file - {:?} ", other_error);
            }
        },
    };
}
