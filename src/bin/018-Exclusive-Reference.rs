fn main() {
    let mut s1 = String::from("hello");

    let r1  = &mut s1;

    change(&mut s1);

    println!("s1 is {}", s1);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
