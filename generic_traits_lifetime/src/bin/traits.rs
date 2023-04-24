use generic_traits_lifetime::{notify, NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("Siddharth"),
        content: String::from("Writing more code is found to improve coding :)"),
        reply: false,
        retweet: false,
    };

    println!("Summarize of tweet: {:}", tweet.summarize());

    let news_article = NewsArticle {
        headline: String::from("Al kills software jobs"),
        location: String::from("Worldwide"),
        author: String::from("ChatGPT"),
        content: String::from("Till 2040 no jobs left for humans"),
    };

    println!("Summarize of news article: {:}", news_article.summarize());
    notify(&tweet);
}
