use std::{cmp::Ordering, env, fs, io::Error, ops::Add};

// Stores args length for pattern matching in file.
const ARGS_LEN: usize = 3;

// To store pattern matching config.
#[derive(PartialEq, Debug)]
pub struct Config {
    pub pattern: String,
    pub file_path: String,
    pub ignore_case: bool,
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
            _ => {
                let ignore_case = env::var("IGNORE_CASE").is_ok();
                Ok(Self {
                    pattern: args[1].clone(),
                    file_path: args[2].clone(),
                    ignore_case,
                })
            }
        }
    }
}

pub fn run(config: Config) -> Result<Vec<(usize, String)>, Error> {
    // String data from file.
    let content: String = fs::read_to_string(config.file_path)?;
    if config.ignore_case {
        search_case_insensitive(config.pattern, content)
    } else {
        search_case_sensitive(config.pattern, content)
    }
}

pub fn search_case_insensitive(
    pattern: String,
    content: String,
) -> Result<Vec<(usize, String)>, Error> {
    let pattern = pattern.to_lowercase();
    // Store line no. & line in tuple.
    let mut result_vec: Vec<(usize, String)> = vec![];
    // Match pattern line by line.
    for (count, line) in content.lines().enumerate() {
        if line.to_lowercase().contains(&pattern) {
            result_vec.push((count.add(1), line.to_string()));
        } else {
            continue;
        }
    }
    Ok(result_vec)
}

pub fn search_case_sensitive(
    pattern: String,
    content: String,
) -> Result<Vec<(usize, String)>, Error> {
    // Store line no. & line in tuple.
    let mut result_vec: Vec<(usize, String)> = vec![];
    // Match pattern line by line.
    for (count, line) in content.lines().enumerate() {
        if line.contains(&pattern) {
            result_vec.push((count.add(1), line.to_string()));
        } else {
            continue;
        }
    }
    Ok(result_vec)
}

#[cfg(test)]
pub mod test {
    use super::*;

    fn sample_config() -> Config {
        Config {
            pattern: String::from("Siddharth"),
            file_path: String::from("test.txt"),
            ignore_case: false,
        }
    }

    #[test]
    fn test_simple_pattern() {
        let mut config = sample_config();
        config.pattern = String::from("Hello");
        assert_eq!(
            run(config).unwrap(),
            vec![(1_usize, "Hello How are you".to_string())]
        );
    }
    #[test]
    fn test_file_not_found() {
        let mut config = sample_config();
        config.file_path = String::from("test1.txt");
        assert!(run(config).is_err());
    }

    #[test]
    fn test_non_existent_pattern() {
        let mut config = sample_config();
        config.pattern = String::from("abadvdsvsd");
        assert_eq!(run(config).unwrap(), vec![]);
    }

    #[test]
    fn test_invalid_args_length() {
        assert_eq!(
            Config::build(&["Siddharth".to_string()]),
            Err("Minimum expected args 3, got 1".to_string())
        );
    }

    #[test]
    fn test_valid_args_length() {
        assert_eq!(
            Config::build(&[
                "main.rs".to_string(),
                "Siddharth".to_string(),
                "test.txt".to_string()
            ]),
            Ok(Config {
                pattern: String::from("Siddharth"),
                file_path: String::from("test.txt"),
                ignore_case: false,
            })
        );
    }
}
