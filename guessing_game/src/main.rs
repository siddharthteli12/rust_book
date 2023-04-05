use std::io::stdin;
use rand::{Rng, thread_rng};
fn main() {
    // Gen. random number.
    let mut rng = thread_rng();
    let random_num: i32 = rng.gen_range(1..100);

    let mut user_guess = 0;

    // Take user input till guess is right.
    while random_num != user_guess {
        // Take user input.
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).unwrap();


        user_guess = match input_string.trim().to_string().parse::<i32>() {
            Err(e) => panic!("Not valid integer `{:}", e),
            Ok(val) => val,
        };
        if user_guess > random_num {
            println!("Guess value too big");
        }
        else if user_guess < random_num {
            println!("Guess value too small");
        }
        else {
            println!("Congratulation right guess");
        }

    }

}
