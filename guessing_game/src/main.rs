use rand::{thread_rng, Rng};
use std::{cmp::Ordering, io::stdin};
fn main() {
    // Gen. random number.
    let mut rng = thread_rng();
    let random_num: i32 = rng.gen_range(1..100);

    let mut user_guess: i32;

    // Take user input till guess is right.
    loop {
        // Take user input.
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).unwrap();

        user_guess = match input_string.trim().to_string().parse::<i32>() {
            Err(e) => panic!("Not valid integer `{:}", e),
            Ok(val) => val,
        };

        match user_guess.cmp(&random_num) {
            Ordering::Greater => println!("Guess value too big"),
            Ordering::Less => println!("Guess value too small"),
            Ordering::Equal => {
                println!("Congratulation right guess");
                break;
            }
        }
    }
}
