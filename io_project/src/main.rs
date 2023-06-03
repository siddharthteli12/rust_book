use io_project::{run, Config};
use std::{env, process};

fn main() {
    match Config::build(env::args().into_iter()) {
        Ok(config) => match run(config) {
            Ok(result_vec) => {
                for result in result_vec {
                    eprintln!("{:} {:}", result.0, result.1);
                }
            }
            Err(e) => {
                eprintln!("Can't find pattern in file due to- {:?}", e);
                process::exit(1)
            }
        },
        Err(e) => {
            eprintln!("Can't build Config due to- {:?}", e);
            process::exit(2)
        }
    }
}
