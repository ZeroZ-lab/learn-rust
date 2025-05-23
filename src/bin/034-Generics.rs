struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2= Point { x: 1.0, y: 2.0 };

    println!("p1: x = {}, y = {}", p1.x, p1.y);
    println!("p2: x = {}, y = {}", p2.x, p2.y);
}
