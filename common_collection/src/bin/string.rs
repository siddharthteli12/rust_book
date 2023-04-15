fn main() {
    // Ascii string.
    let mut simple_string = String::from("SiddHARTH");
    // UTF string.
    let complex_string = String::from("Здравству");

    // Confirm if strings are ascii or not.
    println!(
        "Is simple_string ascii - {:} \nIs complex string ascii - {:}",
        simple_string.is_ascii(),
        complex_string.is_ascii()
    );

    // Length of ascci & UTF string.
    println!(
        "Length ascii string - {:} \nLength UTF string - {:}",
        simple_string.len(),
        complex_string.len()
    );

    // Push UTF string to ascii string.
    simple_string.push_str(&complex_string);
    // Check if simple string is ascii.
    println!("Is simple_string ascii - {:}", simple_string.is_ascii(),);

    // Simple string is mix of ascci & UTF char now.
    // Siddharth is stored as ascii taking 1 byte storage,
    // While Здравству as UTF taking 2 bytes.
    println!(
        "Length ascii string - {:} \nLength UTF string - {:}",
        simple_string.len(),
        complex_string.len()
    );
}
