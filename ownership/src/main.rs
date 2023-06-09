fn main() {
    test_move();
    test_copy();
    assert_eq!(first_word(&"Test 1".to_string()), "Test");
}

// Test the concept of move.
fn test_move() {
    // Value is not moved for prmitive type.
    // Rather new value 100 is stored on stake & y points to it.
    let x: i32 = 100;
    let _y = x;
    println!("Is X valid - {:?}", x);

    // Same doesn't happen with string type.
    // value is moved, ownership is transferred.
    let string_1 = String::from("Siddharth");
    let _string_2 = string_1;
    // Error - value borrowed here after move
    // println!("Is string_1 valid - {:?}", string_1);
}

// Cannot impl Copy trait because of string type which impl drop.
struct Student {
    age: i32,
    height: i32,
    name: String,
}

#[derive(Debug, Copy, Clone)]
struct Student1 {
    pub age: i32,
    pub height: i32,
}

// Test copy trait on stucts.
// Note: All primitive types which are stored on stack impl Copy trait.
// Hence, values are not moved but copied.
// Copy trait cannot be impl for types which imppl drop trait.
fn test_copy() {
    let student = Student {
        age: 23,
        height: 170,
        name: String::from("Siddharth"),
    };

    let student1 = Student1 {
        age: 23,
        height: 170,
    };

    // Try to move value to new variable.
    let _student_new = student;
    let _student_new1 = student1;

    // Invalid student doesn't impl copy trait.
    //println!("Is student valid {:?}", student);

    // Valid student1 impl copy trait.
    println!("Is student1 valid {:?}", student1);
}

fn first_word(s: &String) -> &str {
    let char_list: Vec<char> = s.chars().collect();
    match char_list.iter().position(|&x| x == ' ') {
        Some(n) => &s[0..n],
        None => &s,
    }
}

#[cfg(test)]
mod test {
    use crate::first_word;
    #[test]
    fn test_first_word_with_whitespace() {
        let test_string = String::from("Siddharth Teli");
        let index = first_word(&test_string);
        assert_eq!(index, String::from("Siddharth"));
    }

    #[test]
    fn test_firsttest_first_word_without_whitespace_word() {
        let test_string = String::from("SiddharthTeli");
        let index = first_word(&test_string);
        assert_eq!(index, String::from("SiddharthTeli"));
    }
}
