use std::vec::Vec;

fn main() {
    let mut v_empty: Vec<i32> = Vec::new();

    v_empty.push(1);
    v_empty.push(2);
    v_empty.push(3);

    let third = v_empty[2];

    println!("v_empty: {:?}", v_empty);
    println!("third: {}", third);
}