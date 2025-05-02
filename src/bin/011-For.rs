fn main() {
    for i in 0..10 {
        println!("i is {}", i);
    }

    let arr = [1, 2, 3, 4, 5];

    println!("--------------------------------");

    for i in arr {
        println!("i is {}", i);
    }

    println!("--------------------------------");
    for i in arr.iter() {
        println!("i is {}", i);
    }
}
