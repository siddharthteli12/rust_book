use std::collections::HashMap;

#[derive(Debug, Clone)]
enum FriendType {
    Average,
    Good,
    Best,
}

#[derive(Debug, Clone)]
struct Friend {
    age: i32,
    address: String,
    friend_type: FriendType,
}
fn main() {
    // Using hasmap to store friend info.
    let mut friend_info_map: HashMap<String, Friend> = HashMap::new();
    let mut friend_info = Friend {
        age: 22,
        address: "Bhopal".to_string(),
        friend_type: FriendType::Good,
    };
    // Insert friends.
    friend_info_map.insert("Mohit".to_string(), friend_info.clone());

    friend_info.address = "Gwalior".to_string();
    friend_info_map.insert("Mohil".to_string(), friend_info.clone());

    friend_info.address = "Bhopal".to_string();
    friend_info_map.insert("Shivendra".to_string(), friend_info.clone());

    // Retrive friend info back.
    println!("Friend info {:?}", friend_info_map.get("Mohit").unwrap());

    // Insert if value doesn't exist for key.
    friend_info_map
        .entry("Sai".to_string())
        .or_insert(friend_info);

    // Iterate map values.
    for friend in &mut friend_info_map {
        // Update all age member in map.
        friend.1.age = 25;
    }

    // Check if age is updated.
    println!("Map info {:?}", friend_info_map);
}
