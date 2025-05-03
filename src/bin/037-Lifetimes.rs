fn simple_case() {
    let san_zhang = String::from("张三的家"); // 张三数据，有效期到函数结束
    { // 开始一个内部作用域
        let address_note = &san_zhang; // 便签纸，指向张三
                                      // 图书管理员看到 address_note 的有效期在这个内部作用域
                                      // 而 san_zhang 的有效期更长 (整个函数)
                                      // 规则满足：便签纸有效期 <= 张三有效期
        println!("根据便签找到：{}", address_note);
    } // address_note 在这里作废 (有效期结束)
      // 但 san_zhang 还存在

    println!("张三还在：{}", san_zhang);
} // san_zhang 在这里作废 (有效期结束)
// 一切安好！


// <'a> 声明一个名叫 'a 的标签
// x: &'a str, y: &'a str ：输入的两张便签纸都贴上 'a' 标签
// -> &'a str ：返回的便签纸也贴上 'a' 标签
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { 
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    // 简单示例-1
    // simple_case()

    // 复杂示例
    let x = String::from("x");
    let y = String::from("y");
    let result = longest(&x, &y);
    println!("最长字符串是：{}", result);
}
