fn main() {
    println!("--- 例子 1: 处理 Option<String> ---");

    // 尝试两种情况：
    let nickname_option: Option<String> = Some(String::from("小明"));
    // let nickname_option: Option<String> = None;

    println!("当前 nickname_option 的值是: {:?}", nickname_option);

    // 我们想根据是否有昵称来打印不同的问候语

    // --- 使用 if let ---
    if let Some(name) = nickname_option {
        // --- 解释 ---
        // 1. `if let` 检查 `nickname_option` 这个变量的值。
        // 2. 它尝试将这个值与模式 `Some(name)` 进行匹配。
        // 3. 如果 `nickname_option` 的值是 `Some(...)` 变体：
        //    a. 匹配成功！
        //    b. `Some` 变体内部包含的值 (这里是 String "小明") 会被提取出来，
        //       并绑定(赋值)给模式中定义的新变量 `name`。
        //    c. 执行这个 `{...}` 代码块。
        // 4. 如果 `nickname_option` 的值是 `None` 变体：
        //    a. 匹配失败！
        //    b. 跳过这个 `{...}` 代码块，执行 `else` 部分（如果存在）。

        println!("你好, {}!", name); // 在这个块里，`name` 变量可用，持有 "小明"
    } else {
        // 当 `nickname_option` 是 `None` 时，执行这里
        println!("你好, 陌生人!");
    }

    // 注意：如果 nickname_option 是 Some(String)，并且我们在 if let 块内部
    // 对 name 做了消耗所有权的操作，那么原始的 nickname_option 在之后可能
    // 无法完全使用。但在上面的 println! 中，name 只是被借用了，所以 nickname_option 没事。
    // println!("检查后 nickname_option 的值是: {:?}", nickname_option);
}
