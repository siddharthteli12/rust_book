#![allow(dead_code)]
struct Shoe {
    id: u128,
    size: u8,
    style: ShoeStyle,
}

impl Shoe {
    fn get_size(shoe_collection: Vec<Shoe>, size: u8) -> Vec<Self> {
        shoe_collection
            .into_iter()
            .filter(|shoe| shoe.size == size)
            .collect()
    }

    fn get_style(shoe_collection: Vec<Shoe>, style: ShoeStyle) -> Vec<Self> {
        shoe_collection
            .into_iter()
            .filter(|shoe| shoe.style == style)
            .collect()
    }
}

#[derive(PartialEq)]
enum ShoeStyle {
    Loafer,
    Sneakers,
    Derby,
    Boot,
}

fn main() {}
