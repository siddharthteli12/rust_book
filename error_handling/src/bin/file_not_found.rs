use std::{fs::File, io::ErrorKind};

fn main() {
    let test_file = File::open("test.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("test.txt").unwrap_or_else(|error| {
                panic!("Error creating file {:?}", error);
            })
        } else {
            panic!("{:?}", error);
        }
    });
}
