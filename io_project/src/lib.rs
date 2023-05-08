use std::{cmp::Ordering, fs, io::Error, ops::Add};

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

pub fn handle_pattern_matching(
    pattern: &str,
    file_path: &str,
) -> Result<Vec<(usize, String)>, Error> {
    // String data from file.
    let data: String = fs::read_to_string(file_path)?;
    // Store line no. & line in tuple.
    let mut result_vec: Vec<(usize, String)> = vec![];
    // Match pattern line by line.
    for (count, line) in data.lines().enumerate() {
        if line.contains(pattern) {
            result_vec.push((count.add(1), String::from(line)));
        } else {
            continue;
        }
    }
    Ok(result_vec)
}

#[cfg(test)]
pub mod test {
    use crate::handle_pattern_matching;
    #[test]
    fn test_file_not_found() {
        assert!(handle_pattern_matching("Siddharth", "test1.txt").is_err());
    }

    #[test]
    fn test_non_existent_pattern() {
        assert_eq!(
            handle_pattern_matching("abadvdsvsd", "test.txt").unwrap(),
            vec![]
        );
    }

    #[test]
    fn test_existent_pattern_with_multiple_empty_lines() {
        assert_eq!(
            handle_pattern_matching("Siddharth", "test.txt").unwrap(),
            vec![(8_usize, "Siddharth Is Fine?".to_string())]
        );
    }
}
