use io_project::handle_pattern_matching;
use std::cmp::Ordering;
use std::env;

const ARGS_LEN: usize = 3;

fn main() {
    match read_cli_arguments() {
        Ok((pattern, file_path)) => match handle_pattern_matching(&pattern, &file_path) {
            Ok(result_vec) => {
                for result in result_vec {
                    println!("{:} {:}", result.0, result.1);
                }
            }
            Err(e) => {
                println!("Issue in matching pattern due to {:?}", e);
            }
        },
        Err(e) => {
            println!("Issue with cli arguments due to {:?}", e);
        }
    }
}

fn read_cli_arguments() -> Result<(String, String), std::io::Error> {
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
        Ordering::Equal => Ok((String::from(args[1].clone()), String::from(args[2].clone()))),
    }
}
