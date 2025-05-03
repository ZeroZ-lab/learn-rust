use std::thread;
use std::time::Duration;

fn main() {
    println!("主线程开始");

    // 启动一个新线程
    // 使用 move 关键字，这样闭包会获取它使用的任何外部变量的所有权
    let handle = thread::spawn(move || {
        println!("生成线程：开始工作...");
        for i in 1..=5 {
            println!("生成线程：计数 {}", i);
            thread::sleep(Duration::from_millis(500)); // 模拟耗时工作
        }
        println!("生成线程：工作完成。");
        "来自生成线程的结果".to_string() // 闭包可以返回值
    });

    // 主线程继续执行自己的任务
    for i in 1..=3 {
        println!("主线程：计数 {}", i);
        thread::sleep(Duration::from_millis(300));
    }

    // 等待生成线程结束
    // handle.join() 会阻塞当前（主）线程，直到 handle 代表的线程执行完毕
    // unwrap() 用于处理 join 可能返回的 Err (如果子线程 panic 了)
    match handle.join() {
        Ok(result) => println!("主线程：生成线程已结束，返回结果: '{}'", result),
        Err(_) => println!("主线程：生成线程 panic 了！"),
    }

    println!("主线程结束");
}