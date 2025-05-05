use std::collections::HashMap;

// 声明一个宏 (使用 macro_rules!)
// 这是元编程的一种形式：我们正在编写能生成 Rust 代码的代码。
#[macro_export] // 导出宏，使其可以在其他地方（如此文件的 main 函数）使用
macro_rules! create_map {
    // 匹配模式：以逗号分隔的 0 个或多个 `key => value` 对
    // $key:expr 和 $value:expr 表示捕获表达式作为 key 和 value
    // $(...),* 表示匹配 0 次或多次逗号分隔的模式
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        // 宏展开后的代码块
        {
            // 创建一个新的 HashMap
            let mut map = HashMap::new();
            // 对每一个匹配到的 `key => value` 对...
            $( 
                // ...生成插入 map 的代码
                map.insert($key, $value);
            )*
            // 返回填充好的 map
            map
        }
    };
}

fn main() {
    println!("--- 使用声明宏进行元编程 ---");

    // 使用我们定义的宏来创建 HashMap
    // 编译时，`create_map!` 宏会展开成实际创建和填充 HashMap 的代码
    let map1 = create_map!("a" => 1, "b" => 2, "c" => 3);
    println!("Map 1 (string keys): {:?}", map1);

    let map2 = create_map!(1 => "one", 2 => "two");
    println!("Map 2 (integer keys): {:?}", map2);

    // 也可以创建空 map
    let empty_map: HashMap<i32, i32> = create_map!();
    println!("Empty map: {:?}", empty_map);

    // 带有尾随逗号的版本也能工作
    let map3 = create_map!{
        "apple" => 0.5,
        "banana" => 0.2,
        "orange" => 0.8,
    };
    println!("Map 3 (trailing comma): {:?}", map3);
} 