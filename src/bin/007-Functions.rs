fn main() {
    let result = add(1, 2);

    println!("result is {}", result);

    print_number(result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print_number(number: i32) {
    println!("number is {}", number);
}
