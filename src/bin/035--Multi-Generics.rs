struct Point<T, U> {
    x: T,
    y: U,
}


fn main() {
    let p1 = Point { x: 1, y: 2.0 };
    let p2 = Point { x: "Hello", y:"World" };

    println!("p1: x = {}, y = {}", p1.x, p1.y);
    println!("p2: x = {}, y = {}", p2.x, p2.y);
}