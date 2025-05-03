use std::vec::Vec;

fn main() {
    let v_empty: Vec<i32> = Vec::new();

    let v_init: Vec<i32> = vec![1, 2, 3];

    println!("v_empty: {:?}", v_empty);
    println!("v_init: {:?}", v_init);
}