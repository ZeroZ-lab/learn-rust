fn main() {
    
    // 生成斐波那契数列的第 n 项。    
    let n = 10;

    let result = fibonacci(n);

    println!("result is {}", result);
    
    
}

fn fibonacci(n: i32) -> i32 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

