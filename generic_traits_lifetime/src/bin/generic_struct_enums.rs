mod rectangle {
    use std::ops::Mul;

    #[derive(Clone, Copy)]
    pub struct Rectangle<T, U> {
        height: T,
        width: U,
    }

    impl<'a, 'b: 'a, T: 'a, U: 'b> Rectangle<T, U>
    where
        &'b U: Mul<&'a T, Output = U>,
    {
        pub fn new(height: T, width: U) -> Self {
            Self { height, width }
        }

        pub fn area(&'b self) -> U {
            &self.width * &self.height
        }

        pub fn height(self) -> T {
            self.height
        }

        pub fn width(self) -> U {
            self.width
        }
    }
}

use rectangle::Rectangle;

fn main() {
    let rec_1 = Rectangle::new(100, 200);
    println!(
        "Recetangle\nheight - {:}\nwidth - {:}\narea - {:}",
        rec_1.height(),
        rec_1.width(),
        rec_1.area()
    );
}
