

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("some_number is {:?}", some_number);
    println!("some_string is {:?}", some_string);
    println!("absent_number is {:?}", absent_number);

}
