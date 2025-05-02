fn main() {
    let tuple: (i32, i32, i32) = (1, 2, 3);

    // `:?` 是 Rust 中的调试（Debug）格式化说明符。
    // 它告诉 `println!` 宏使用 `tuple` 变量的 `Debug` trait 实现来格式化输出。
    // 这对于打印复杂数据结构（如元组、结构体、枚举等）的开发者友好表示非常有用。
    println!("tuple is {:?}", tuple); // 使用 Debug trait 打印元组内容

    let (x, y, z) = tuple;

    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
}
