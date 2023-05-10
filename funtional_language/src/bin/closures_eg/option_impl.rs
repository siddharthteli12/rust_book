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
}

pub fn custom_option() {
    let value1 = Some1(1);
    let value2 = None1;
    println!("Value1 - {:}", value1.unwrap_or_else(|| 5));
    println!("Value2 - {:}", value2.unwrap_or_else(|| 10));
}
