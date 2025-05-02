struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width: width, height: height }
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle::new(30, 50);
    println!("rect1 is {}", rect1.area());
}



