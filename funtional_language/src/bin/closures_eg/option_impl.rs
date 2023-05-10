pub enum CustomOption<T> {
    Some1(T),
    None1,
}

use CustomOption::{None1, Some1};
impl<T> CustomOption<T> {
    fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Some1(val) => val,
            None1 => f(),
        }
    }

    fn unwrap(self) -> T {
        match self {
            Some1(val) => val,
            None1 => panic!("Called unwarp on none value"),
        }
    }
}

pub fn custom_option() {
    let value1 = Some1(1);
    let value2 = None1;
    let value3 = Some1(10);
    println!("Value1 - {:}", value1.unwrap_or_else(|| 5));
    println!("Value2 - {:}", value2.unwrap_or_else(|| 10));
    println!("Value1 - {:}", value3.unwrap());
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn test_unwarp_on_none() {
        let test_value: CustomOption<i32> = None1;
        test_value.unwrap();
    }

    #[test]
    fn test_unwarp_on_some() {
        let test_value: CustomOption<i32> = Some1(10);
        assert_eq!(test_value.unwrap(), 10);
    }

    #[test]
    fn test_unwarp_else_on_none() {
        let test_value: CustomOption<i32> = None1;
        assert_eq!(test_value.unwrap_or_else(|| 500), 500);
    }

    #[test]
    fn test_unwarp_else_on_some() {
        let test_value: CustomOption<i32> = Some1(123);
        assert_eq!(test_value.unwrap_or_else(|| 500), 123);
    }
}
