use io_project::handle_pattern_matching;
use std::io;

fn main() {
    let mut input_data = String::new();
    io::stdin()
        .read_line(&mut input_data)
        .expect("Issue reading input");
    let (pattern, file_path) = input_data.rsplit_once(" ").unwrap();

    match handle_pattern_matching(pattern.trim(), file_path.trim()) {
        Ok(result_vec) => {
            for result in result_vec {
                println!("{:} {:}", result.0, result.1);
            }
        }
        Err(e) => {
            println!("Issue in matching pattern due to {:?}", e);
        }
    }
}
