use generic_traits_lifetime::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("Siddharth"),
        content: String::from("Writing more code is found to improce coding :)"),
        reply: false,
        retweet: false,
    };

    println!("Summarize of tweet: {:}", tweet.summarize());
}
