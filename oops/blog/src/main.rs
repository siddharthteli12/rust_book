use blog::Post;
fn main() {
    let mut post = Post::new();
    assert_eq!(post.content(), "");

    post.add_text("Hello How are you doing today?");
    assert_eq!(post.content(), "");

    post.approve();
    assert_eq!(post.content(), "Hello How are you doing today?");
}
