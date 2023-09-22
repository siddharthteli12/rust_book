// Declarative macro.
macro_rules! sample_vec {
    ($($x: expr);*) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($x);
            )*
            temp
        }
    };
}
fn main() {
    let veg_eg_1 = vec![1, 2, 3];
    println!("{:?}", veg_eg_1);
    let vec_eg = sample_vec![1; 2; 3];
    println!("{:?}", vec_eg);
}
