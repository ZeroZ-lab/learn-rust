use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个通道，类型是 String
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    // 克隆发送端，给第一个子线程
    let tx1 = tx.clone();
    thread::spawn(move || {
        // move 捕获 tx1
        let messages = vec!["线程1", "你好", "世界"];
        for msg in messages {
            let owned_msg = msg.to_string(); // 创建拥有的 String
            println!("Tx1 发送: {}", owned_msg);
            tx1.send(owned_msg).unwrap(); // 发送，所有权转移
            thread::sleep(Duration::from_secs(1));
        }
        println!("Tx1 发送完毕，即将关闭。");
        // tx1 在此 drop
    });

    // 使用原始发送端，给第二个子线程
    thread::spawn(move || {
        // move 捕获 tx
        let messages = vec!["线程2", "发送", "更多消息"];
        for msg in messages {
            let owned_msg = msg.to_string();
            println!("Tx 发送: {}", owned_msg);
            tx.send(owned_msg).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
        println!("Tx 发送完毕，即将关闭。");
        // tx 在此 drop
    });

    // 主线程接收消息
    println!("主线程：等待接收消息...");
    // 可以直接迭代 Receiver，它会在通道关闭且为空时结束
    for received_msg in rx {
        println!("主线程 收到: {}", received_msg);
    }

    println!("主线程：接收端关闭（因为所有发送端都已关闭）。");
}
