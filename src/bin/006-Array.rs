fn main() {
    let arr: [i32; 3] = [1, 2, 3];

    println!("arr is {:?}", arr);

    let [x, y, z] = arr;

    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
}
