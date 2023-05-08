use io_project::{search_pattern, Config};
use std::{env, process};

fn main() {
    match Config::build(&env::args().collect::<Vec<String>>()) {
        Ok(config) => match search_pattern(config) {
            Ok(result_vec) => {
                for result in result_vec {
                    println!("{:} {:}", result.0, result.1);
                }
            }
            Err(e) => {
                println!("Can't find pattern in file due to- {:?}", e);
                process::exit(1)
            }
        },
        Err(e) => {
            println!("Can't build Config due to- {:?}", e);
            process::exit(2)
        }
    }
}
