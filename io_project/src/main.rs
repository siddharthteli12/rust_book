use io_project::handle_pattern_matching;
use std::cmp::Ordering;
use std::env;

// Stores args length for pattern matching in file.
const ARGS_LEN: usize = 3;

// To store pattern matching config.
pub struct Config {
    pub pattern: String,
    pub file_path: String,
}

impl Config {
    // Calls validate & then creates config instance.
    pub fn new(args: &[String]) -> Self {
        Self::validate_config_args(args);
        Self {
            pattern: args[1].clone(),
            file_path: args[2].clone(),
        }
    }

    // Validates arg length is correct or else panics.
    pub fn validate_config_args(args: &[String]) {
        match args.len().cmp(&ARGS_LEN) {
            Ordering::Equal => return,
            _ => panic!("Expected {:} arg, found {:} arg", ARGS_LEN, args.len()),
        };
    }
}

fn main() {
    let config = Config::new(&env::args().collect::<Vec<String>>());
    // Matches given pattern in given file.
    match handle_pattern_matching(&config.pattern, &config.file_path) {
        Ok(result_vec) => {
            for result in result_vec {
                println!("{:} {:}", result.0, result.1);
            }
        }
        Err(e) => {
            println!("Can't find pattern in file due to- {:?}", e);
        }
    }
}
