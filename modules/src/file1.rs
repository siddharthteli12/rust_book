pub fn check_div(x: &i32, y: &i32) -> bool {
    if y > x {
        false
    } else {
        true
    }
}

pub mod maths {
    pub fn add(x: &i32, y: &i32) -> i32 {
        x + y
    }

    pub fn sub(x: &i32, y: &i32) -> i32 {
        x - y
    }

    pub fn mul(x: &i32, y: &i32) -> i32 {
        x * y
    }

    pub fn div(x: &i32, y: &i32) -> i32 {
        x / y
    }
}
