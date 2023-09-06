/*
Eg 1.
Default generic type trait.

*/
trait IteratorAdd<Rhs = Self> {
    type Output;
    fn iter_both(self, rhs: Rhs) -> Option<Self::Output>;
}

impl IteratorAdd<Vec<f64>> for Vec<i32> {
    type Output = Vec<f64>;
    fn iter_both(self, rhs: Vec<f64>) -> Option<Self::Output> {
        let mut output: Vec<f64>;
        if self.len() > rhs.len() {
            output = self.iter().map(|item| *item as f64).collect();
            rhs.iter().enumerate().for_each(|(index, item)| {
                output[index] += item;
            });
        } else {
            output = rhs;
            self.iter()
                .map(|item| *item as f64)
                .enumerate()
                .for_each(|(index, item)| {
                    output[index] += item;
                })
        }

        Some(output)
    }
}

/*
Eg 2.
Same method differnet trait, using ful syntax to differentiate.

*/

pub trait TestTrait1 {
    fn test_method(&self) -> i32;
}

pub trait TestTrait2 {
    fn test_method(&self) -> i32;
}

pub trait TestTrait3 {
    fn test_method(&self) -> i32;
}

struct TestStruct;

impl TestTrait1 for TestStruct {
    fn test_method(&self) -> i32 {
        1
    }
}

impl TestTrait2 for TestStruct {
    fn test_method(&self) -> i32 {
        2
    }
}

impl TestTrait3 for TestStruct {
    fn test_method(&self) -> i32 {
        3
    }
}

/*
Eg 3.
Same method differnet trait, using ful syntax to differentiate but without self.

*/

pub trait Trait1 {
    fn test_method() -> i32;
}

pub trait Trait2 {
    fn test_method() -> i32;
}

pub trait Trait3 {
    fn test_method() -> i32;
}

struct Struct;

impl Trait1 for Struct {
    fn test_method() -> i32 {
        1
    }
}

impl Trait2 for Struct {
    fn test_method() -> i32 {
        2
    }
}

impl Trait3 for Struct {
    fn test_method() -> i32 {
        3
    }
}

#[cfg(test)]
mod test_eg_1 {
    use super::*;

    #[test]
    fn with_simple_list() {
        let list1 = vec![1, 2, 3, 4, 5];
        let list2 = vec![1.2, 2.5, 2.7, 8.9];
        assert_eq!(list1.iter_both(list2), Some(vec![2.2, 4.5, 5.7, 12.9, 5.0]));
    }

    #[test]
    fn with_simple_list_2() {
        let list1 = vec![10, 12, 13];
        let list2 = vec![9.7, 4.54, 45.45, 343.0, 23.0, 12.0];
        assert_eq!(
            list1.iter_both(list2),
            Some(vec![19.7, 16.54, 58.45, 343.0, 23.0, 12.0])
        );
    }
}

#[cfg(test)]
mod test_eg_2 {
    use super::*;

    #[test]
    fn should_work() {
        let test_struct = TestStruct;
        // Compiler can't find which test_method to call here.
        // test_struct.test_method(); -> Error.

        assert_eq!(TestTrait1::test_method(&test_struct), 1);
        assert_eq!(TestTrait2::test_method(&test_struct), 2);
        assert_eq!(TestTrait3::test_method(&test_struct), 3);
    }
}

#[cfg(test)]
mod test_eg_3 {
    use super::*;

    #[test]
    fn should_work_1() {
        // Above signature is invalid now.
        // Compiler can't find for which type trait method is to be called for.
        // TestTrait1::test_method()

        // <Type as Trait>::function_signature is used to associate trait & caller type.
        assert_eq!(<Struct as Trait1>::test_method(), 1);
        assert_eq!(<Struct as Trait2>::test_method(), 2);
        assert_eq!(<Struct as Trait3>::test_method(), 3);
    }
}
