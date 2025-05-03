pub fn greet() {
    println!("Hello from my_module!");
}

#[allow(dead_code)] // 避免未使用函数的警告
fn private_function() {
    println!("This is private.");
} 