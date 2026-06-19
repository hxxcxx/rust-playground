//! 19.3 通道：mpsc / sync_channel / 多生产者 / 管道
//!
//! 关键结论：
//! - 「通道」是线程间传递消息的方式 —— 用「通信」代替「共享内存」。
//! - `std::sync::mpsc`：
//!   * `channel()` —— 异步、无界（发送方不阻塞，可能堆积）。
//!   * `sync_channel(n)` —— 同步、有界（缓冲满时发送方阻塞）。
//! - 多生产者：`tx.clone()` 得到多个发送端（mpsc 的 m = multiple）。
//! - 单消费者：rx 只能有一个（所有权）。
//! - 接收方 `rx.recv()` 阻塞等；`rx.try_recv()` 非阻塞；`rx.iter()` 持续收。
//! - 所有发送端 drop 后，rx 收到 None（迭代结束）。
//! - 经典模式：工作队列、流水线、扇出/扇入。
//!
//! 运行：`cargo run -p ch19_concurrency --example 03_channels`

use ch19_concurrency::section;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    section("基本通道：单生产者单消费者");
    // channel() 返回 (sender, receiver)。T 是发送值的类型。
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        // 子线程发送，然后退出（tx 被 drop → 通道关闭）。
        for i in 0..3 {
            tx.send(i).unwrap();
            println!("    发送: {i}");
        }
        // tx 离开作用域被 drop。
    });
    // 主线程接收。recv() 阻塞直到有值或通道关闭。
    while let Ok(v) = rx.recv() {
        println!("  收到: {v}");
    }
    println!("  （通道关闭，recv 返回 Err）");

    section("rx.iter()：持续接收直到关闭");
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        let msgs = ["hello", "world", "done"];
        for m in msgs {
            tx.send(m.to_string()).unwrap();
        }
        // tx drop → 通道关闭。
    });
    // iter() 在通道关闭时自然结束。
    for msg in rx {
        println!("  {msg}");
    }

    section("多生产者：tx.clone() 扇出");
    // 多个线程向同一个 rx 发送 —— 经典的「工作收集」模式。
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    for worker in 0..3 {
        let tx = tx.clone(); // 克隆发送端
        handles.push(thread::spawn(move || {
            for task in 0..2 {
                tx.send(format!("worker{worker}-task{task}")).unwrap();
                thread::sleep(Duration::from_millis(10));
            }
        }));
    }
    // drop 原始 tx，否则通道永远不会关闭（还有引用）。
    drop(tx);
    // 主线程收集所有结果。
    let mut all: Vec<String> = rx.iter().collect();
    all.sort(); // 排序让输出稳定（接收顺序随机）。
    for msg in &all {
        println!("  {msg}");
    }
    for h in handles {
        h.join().unwrap();
    }

    section("sync_channel：有界同步通道");
    // sync_channel(2) 缓冲区容量 2 —— 满了发送方阻塞。
    // 适合「背压」（backpressure）：防止生产者远快于消费者导致堆积。
    let (tx, rx) = mpsc::sync_channel(2);
    let producer = thread::spawn(move || {
        for i in 0..5 {
            // 超过容量会阻塞，直到消费者取走。
            tx.send(i).unwrap();
            println!("    生产: {i}");
        }
    });
    // 消费者慢慢取，观察生产者被阻塞。
    thread::sleep(Duration::from_millis(50));
    while let Ok(v) = rx.recv() {
        println!("  消费: {v}");
        thread::sleep(Duration::from_millis(20));
    }
    producer.join().unwrap();

    section("流水线：多阶段串联");
    // 阶段 1 → 阶段 2 → 输出。每阶段一个线程。
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    // 阶段 1：生成数字。
    thread::spawn(move || {
        for i in 0..5 {
            tx1.send(i).unwrap();
        }
    });
    // 阶段 2：平方。
    thread::spawn(move || {
        for n in rx1 {
            tx2.send(n * n).unwrap();
        }
    });
    // 主线程：接收最终结果。
    for result in rx2 {
        println!("  流水线输出: {result}");
    }

    section("传送所有权：通道转移而非共享");
    // send(val) 把 val 的所有权转移到接收线程 —— 这就是「通过通信共享内存」。
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let data = vec![1, 2, 3]; // data 在子线程创建
        tx.send(data).unwrap(); // 所有权转移给主线程
        // println!("{data:?}"); // ❌ data 已被 send 走
    });
    let received: Vec<i32> = rx.recv().unwrap();
    println!("  收到所有权: {received:?}");

    section("通道的错误处理");
    let (tx, rx) = mpsc::channel();
    // 接收方先 drop。
    drop(rx);
    // 发送到已关闭通道 → SendError。
    match tx.send(42) {
        Ok(()) => println!("  发送成功"),
        Err(e) => println!("  发送失败（接收方已关闭）: {e}"),
    }
}
