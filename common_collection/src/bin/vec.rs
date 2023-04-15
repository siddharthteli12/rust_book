use std::vec;

fn main() {
    // List of friends.
    let mut friends_list = vec!["mohit".to_string(), "akhil".to_string(), "jam".to_string()];
    friends_list.push("Sai".to_string());

    // Iter all friends list.
    for friend in &mut friends_list {
        // Each friend to upper case.
        *friend = friend.to_uppercase();
    }

    println!("List {:?}", friends_list);
    // Remove last friend from list.
    friends_list.pop();
    // Assert friend list is same.
    assert_eq!(
        vec!["MOHIT".to_string(), "AKHIL".to_string(), "JAM".to_string()],
        friends_list
    );
}
