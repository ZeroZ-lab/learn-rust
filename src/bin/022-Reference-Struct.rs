struct User<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

fn main() {
    let username = String::from("user1");
    let email = String::from("user1@example.com");

    let user1 = User {
        active: true,
        username: &username,
        email: &email,
        sign_in_count: 1,
    };

    println!("user1 is {}", user1.username);
    println!("user1 is {}", user1.email);
    println!("user1 is {}", user1.sign_in_count);
    println!("user1 is {}", user1.active);
}
