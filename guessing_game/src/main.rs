use rand::{thread_rng, Rng};
use std::{cmp::Ordering, io::stdin};
mod guess;
use guess::Guess;

fn main() {
    // Gen. random number.
    let mut rng = thread_rng();
    let random_num: i32 = rng.gen_range(1..100);

    // String to store user input.
    let mut input_string = String::new();

    // Take user input till guess is right.
    loop {
        // Take user input.
        stdin().read_line(&mut input_string).unwrap();

        // Break when guess is right or continue.
        if handle_user_input(&input_string, random_num) {
            break;
        } else {
            input_string.clear();
            continue;
        }
    }
}

fn handle_user_input(input_string: &str, random_num: i32) -> bool {
    // Construct guess value.
    let user_guess = Guess::new(
        input_string
            .trim()
            .to_string()
            .parse::<i32>()
            .expect("Invalid input"),
    );

    // Match user input against value & revert.
    match user_guess.value().cmp(&random_num) {
        Ordering::Greater => {
            println!("Guess value too big");
            false
        }
        Ordering::Less => {
            println!("Guess value too small");
            false
        }
        Ordering::Equal => {
            println!("Congratulation right guess");
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::Guess;
    #[test]
    #[should_panic(expected = "Value should be between 0 to 100 only")]
    fn test_guess_invalid_value_1() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Value should be between 0 to 100 only")]
    fn test_guess_invalid_value_2() {
        Guess::new(-1);
    }

    #[test]
    fn test_guess_valid_value_1() {
        Guess::new(50);
    }
}
