use rand::{thread_rng, Rng};
use std::{cmp::Ordering, io::stdin};

mod guess {
    pub struct Guess(i32);

    impl Guess {
        pub fn new(value: i32) -> Self {
            if value < 0 || value > 100 {
                panic!("Value should be between 0 to 100 only");
            }
            Self(value)
        }
        pub fn value(self) -> i32 {
            self.0
        }
    }
}

use guess::Guess;

fn main() {
    // Gen. random number.
    let mut rng = thread_rng();
    let random_num: i32 = rng.gen_range(1..100);

    let mut user_guess: Guess;
    // Take user input till guess is right.
    loop {
        // Take user input.
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).unwrap();

        user_guess = match input_string.trim().to_string().parse::<i32>() {
            Err(e) => panic!("Not valid integer `{:}", e),
            Ok(val) => Guess::new(val),
        };

        match user_guess.value().cmp(&random_num) {
            Ordering::Greater => println!("Guess value too big"),
            Ordering::Less => println!("Guess value too small"),
            Ordering::Equal => {
                println!("Congratulation right guess");
                break;
            }
        }
    }
}
