use std::thread;
use std::time::Duration;

fn main() {
    let intensity = 10;
    let random_number = 7;

    println!("计算锻炼计划...");
    generate_workout(intensity, random_number);

    // --- 基本闭包定义与类型推断 ---
    // 编译器推断 `num` 和返回值的类型。
    let add_one = |num| num + 1;
    let five = add_one(4);
    println!("4 + 1 = {}", five);

    // 显式类型注解 (通常不需要)
    let add_two = |num: u32| -> u32 { num + 2 };
    let six = add_two(4);
    println!("4 + 2 = {}", six);

    // --- 捕获环境 ---

    // 通过不可变引用捕获 `x` (隐式)
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
    println!("{} 等于 {}? {}", y, x, equal_to_x(y));
    // x 仍然可以在这里使用，因为闭包只不可变地借用了它
    println!("x 仍然是 {}", x);

    // 通过可变引用捕获 `vec` (隐式)
    let mut vec = vec![1, 2, 3];
    let mut push_to_vec = || {
        vec.push(4);
        println!("闭包内部, vec: {:?}", vec);
    };
    // println!("不能在这里使用 vec: {:?}", vec); // 错误：无法不可变地借用 `vec`，因为它已经被可变地借用了
    push_to_vec();
    push_to_vec(); // 可以多次调用，因为它可变地借用
    // 可变借用在最后一次调用 push_to_vec 后结束
    println!("闭包之后, vec: {:?}", vec);

    // 通过值捕获 (move)
    let v = vec![1, 2, 3];
    println!("定义闭包之前: {:?}", v);
    // 使用 `move` 关键字强制闭包获取 `v` 的所有权
    let takes_ownership = move || println!("move 闭包内部: {:?}", v);
    // println!("定义闭包之后: {:?}", v); // 错误！ `v` 的所有权已被移入闭包
    takes_ownership();
    // takes_ownership(); // 可以再次调用

}

// --- 使用闭包的函数 (来自 Rust 官方文档的例子) ---

// 模拟一个耗时的计算
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("缓慢计算中...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// 生成锻炼计划，使用闭包避免冗余计算
fn generate_workout(intensity: u32, random_number: u32) {
    // 定义一个闭包，它持有耗时计算的结果。
    // 它不可变地捕获 `intensity`。
    // 类型由编译器推断。
    let mut expensive_closure = Cacher::new(|num| {
        println!("缓慢计算中...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "今天，做 {} 个俯卧撑!",
            expensive_closure.value(intensity)
        );
        println!(
            "接下来，做 {} 个仰卧起坐!",
            expensive_closure.value(intensity) // 使用缓存的值
        );
    } else {
        if random_number == 3 {
            println!("今天休息！记得保持水分。");
        } else {
            println!(
                "今天，跑步 {} 分钟!",
                expensive_closure.value(intensity)
            );
        }
    }
}

// --- 用于缓存闭包结果的结构体 --- (来自 Rust 官方文档的例子)

// 一个持有闭包并缓存其结果的结构体。
// 需要 Fn trait 约束，因为我们只需要调用闭包一次来获取结果。
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T>
    Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // 获取缓存的值或执行闭包计算新值。
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
