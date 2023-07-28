// This is library crate. Can't be run. But imported in other package as dependency.
// By default there can be min zero or max 1 lib crate.
// Name of lib crate by default is package name.
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
