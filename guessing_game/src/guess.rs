use std::process::exit;

pub struct Guess(i32);

impl Guess {
    pub fn new(value: i32) -> Self {
        if (0..100).contains(&value) {
            Self(value)
        }
        else {
            println!("Value should be between 0 to 100 only");
            exit(0x0100)
        }
    }
    pub fn value(self) -> i32 {
        self.0
    }
}
