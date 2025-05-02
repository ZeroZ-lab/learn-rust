fn main() {
    // 学习 scoop ，变量作用域

    let x = 10;

    {
        let x = 20;
        println!("x is {}", x);
    }

    println!("x is {}", x);
}
