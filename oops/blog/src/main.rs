use blog::{Target, B};

fn main() {
    let mut target = Target::new();
    // Value is A.
    target.output();

    //Value is changed to B.
    target.change_state(Box::new(B));
    target.output();
}
