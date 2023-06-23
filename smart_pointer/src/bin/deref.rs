use std::ops::{Deref, DerefMut};

// Without deref or deref_mut impl.
struct CustomStruct<T>(T);

struct CustomStruct1<T>(T);

impl<T> Deref for CustomStruct1<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CustomStruct1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// We cannot use * operator directly with custom type like struct & enums.
// If we impl deref or deref mut for custom_type.
// Whenever we call, *custom_type deref or deref_mut is called acc.

// Note - Deref or DerefMut is mostly impl for smart pointers.

fn main() {
    let _custom_value = CustomStruct(100);
    let mut custom_value1 = CustomStruct1(100);
    // Type mismatch.
    // assert_eq!(100, *_custom_value);

    // Compiler converts *custom_value1 to *(custom_value1.deref())
    assert_eq!(100, *custom_value1);

    // Compiler converts *custom_value1 to *(custom_value1.deref_mut())
    *custom_value1 = 200;
    assert_eq!(200, *custom_value1);
}
