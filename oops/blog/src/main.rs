use blog::Post;
fn main() {
    let mut post = Post::new();
    post.add_text("Hello how are you?");
    assert_eq!(post.show_content(), "");
    // Calling approve here, will have no effect.
    post.approve();
    assert_eq!(post.show_content(), "");

    // Will change internal state.
    post.request_review();
    assert_eq!(post.show_content(), "");

    // Will change internal state.
    post.approve();
    assert_eq!(post.show_content(), "Hello how are you?");
}
