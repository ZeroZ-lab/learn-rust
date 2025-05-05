// 从我们主项目的库中导入 HelloMacro trait
use learn_rust::HelloMacro;

// 从宏 crate 中导入 derive 宏
// 宏的名称与 trait 名称一致
use hello_macro_derive::HelloMacro;

// 定义一个简单的结构体
#[derive(HelloMacro)] // 使用我们自定义的 derive 宏
struct Pancakes;

// 再定义一个结构体
#[derive(HelloMacro)] // 使用我们自定义的 derive 宏
struct Waffles;

fn main() {
    println!("--- 使用过程宏 (自定义 Derive) ---");

    // 调用由宏生成的 hello_macro 方法
    // 编译时，#[derive(HelloMacro)] 会让 hello_macro_derive 宏运行，
    // 为 Pancakes 结构体生成 `impl HelloMacro for Pancakes { ... }` 代码块。
    Pancakes::hello_macro();

    // Waffles 结构体同样会生成实现
    Waffles::hello_macro();
} 