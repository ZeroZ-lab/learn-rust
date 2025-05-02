struct User {
    active: bool,
    username: String, // 使用拥有所有权的 String 类型
    email: String,    // 同上
    sign_in_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32,
}

struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    println!("user1 is {}", user1.username);
    println!("user1 is {}", user1.email);
    println!("user1 is {}", user1.sign_in_count);
    println!("user1 is {}", user1.active);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1.width);
    println!("rect1 is {}", rect1.height);

    let point1 = Point { x: 1.0, y: 2.0 };

    println!("point1 is {}", point1.x);
    println!("point1 is {}", point1.y);
}
