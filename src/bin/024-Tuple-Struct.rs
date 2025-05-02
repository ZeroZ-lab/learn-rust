struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);

    println!("red is {}", red.0);
    println!("origin is {}", origin.0);
}