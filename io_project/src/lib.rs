use std::{fs, io::Error, ops::Add};

pub fn handle_pattern_matching(
    pattern: &str,
    file_path: &str,
) -> Result<Vec<(usize, String)>, Error> {
    let data: String = fs::read_to_string(file_path)?;
    let mut result_vec: Vec<(usize, String)> = vec![];
    for (count, line) in data.lines().enumerate() {
        if line.contains(pattern) {
            result_vec.push((count.add(1), String::from(line)));
        } else {
            continue;
        }
    }
    Ok(result_vec)
}
