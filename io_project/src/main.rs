use io_project::handle_pattern_matching;
use std::cmp::Ordering;
use std::env;

const ARGS_LEN: usize = 3;

pub struct Config {
    pub pattern: String,
    pub file_path: String,
}

fn main() {
    match read_cli_arguments() {
        Ok(config) => match handle_pattern_matching(&config.pattern, &config.file_path) {
            Ok(result_vec) => {
                for result in result_vec {
                    println!("{:} {:}", result.0, result.1);
                }
            }
            Err(e) => {
                println!("Issue- {:?}", e);
            }
        },
        Err(e) => {
            println!("Issue with cli arguments due to {:?}", e);
        }
    }
}

fn read_cli_arguments() -> Result<Config, std::io::Error> {
    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&ARGS_LEN) {
        Ordering::Less => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Expected {:} arg, found {:} arg", ARGS_LEN, args.len()),
        )),
        Ordering::Greater => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Expected {:} arg, found {:} arg", ARGS_LEN, args.len()),
        )),
        Ordering::Equal => Ok(Config {
            pattern: String::from(args[1].clone()),
            file_path: String::from(args[2].clone()),
        }),
    }
}
