#![allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
struct Shoe {
    id: u128,
    size: u8,
    style: ShoeStyle,
}

impl Shoe {
    fn new(id: u128, size: u8, style: ShoeStyle) -> Self {
        Self { id, size, style }
    }
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

#[derive(PartialEq, Debug, Clone)]
enum ShoeStyle {
    Loafer,
    Sneakers,
    Derby,
    Boot,
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_empty_collection() {
        assert_eq!(Shoe::get_size(vec![], 10), vec![]);
        assert_eq!(Shoe::get_style(vec![], ShoeStyle::Boot), vec![]);
    }

    #[test]
    fn test_with_simple_collection() {
        let collections = vec![
            Shoe::new(1, 10, ShoeStyle::Boot),
            Shoe::new(1, 11, ShoeStyle::Boot),
            Shoe::new(1, 11, ShoeStyle::Loafer),
        ];
        assert_eq!(
            Shoe::get_size(collections.clone(), 10),
            vec![Shoe::new(1, 10, ShoeStyle::Boot)]
        );
        assert_eq!(
            Shoe::get_style(collections, ShoeStyle::Boot),
            vec![
                Shoe::new(1, 10, ShoeStyle::Boot),
                Shoe::new(1, 11, ShoeStyle::Boot)
            ]
        );
    }
}
