// This is a binary crate.
// If a binary crate exist in src/main, It has same name has package in cargo.toml.
// To run this binary crate when other binary crate exist run -
// cargo run --bin managing_growing_project
// Binary crate can't be important in other package.
fn main() {
    println!("Inside src main");
}
