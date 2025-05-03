use std::time::Duration;

// 定义一个异步函数
async fn first_async_task() {
    println!("异步任务 1：开始");
    // 模拟一些异步工作，比如等待 1 秒
    // 注意：这里的 sleep 需要是异步兼容的 sleep，std::thread::sleep 会阻塞！
    // 我们稍后会看到如何使用像 tokio::time::sleep 这样的异步 sleep。
    // 这里暂时只打印信息。
    println!("异步任务 1：完成");
}

fn main() {
    println!("主函数：调用异步函数...");
    let future1 = first_async_task(); // 调用 async fn，返回一个 Future
    println!("主函数：异步函数已调用，得到 Future。但任务尚未执行！");
    // 要执行 future1，我们需要一个执行器 (Executor)
}