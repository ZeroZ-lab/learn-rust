struct User {
    active: bool,
    username: String, // 使用拥有所有权的 String 类型
    email: String,    // 同上
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("change@example.com");

    println!("user1 is {}", user1.username);
    println!("user1 is {}", user1.email);
    println!("user1 is {}", user1.sign_in_count);
    println!("user1 is {}", user1.active);
}
