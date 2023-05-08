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
    // Validate args len & creates config instance.
    pub fn build(args: &[String]) -> Result<Self, String> {
        match args.len().cmp(&ARGS_LEN) {
            Ordering::Less => Err(format!(
                "Minimum expected args {:}, got {:}",
                ARGS_LEN,
                args.len()
            )),
            _ => Ok(Self {
                pattern: args[1].clone(),
                file_path: args[2].clone(),
            }),
        }
    }
}

fn main() {
    match Config::build(&env::args().collect::<Vec<String>>()) {
        Ok(config) => match handle_pattern_matching(&config.pattern, &config.file_path) {
            Ok(result_vec) => {
                for result in result_vec {
                    println!("{:} {:}", result.0, result.1);
                }
            }
            Err(e) => println!("Can't find pattern in file due to- {:?}", e),
        },
        Err(e) => println!("Can't build Config due to- {:?}", e),
    }
}
