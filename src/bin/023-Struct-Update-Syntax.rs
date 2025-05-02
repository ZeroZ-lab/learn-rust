struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };

    println!("user2 is {}", user2.username);
    println!("user2 is {}", user2.email);
    println!("user2 is {}", user2.sign_in_count);
    println!("user2 is {}", user2.active);
}
