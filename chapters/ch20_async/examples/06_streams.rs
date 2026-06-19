//! 20.6 Stream / 异步迭代器 + 异步 channel
//!
//! 关键结论：
//! - `Stream` 是「异步版 Iterator」：`async fn next() -> Option<Item>`。
//! - 标准 trait 在 `futures` crate；tokio 的 channel receiver 自带 Stream 实现。
//! - `tokio::sync::mpsc`：异步多生产者单消费者通道（对应第 19 章的 std mpsc）。
//!   * `tx.send(v).await` —— 异步发送（满时会 await 让出，而非阻塞）。
//!   * `rx.recv().await` —— 异步接收。
//! - `while let Some(v) = stream.next().await` —— 异步迭代。
//! - Tokio 还提供：oneshot（一次性）、broadcast（广播）、watch（观察单个值）。
//!
//! 运行：`cargo run -p ch20_async --example 06_streams`

use ch20_async::section;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    section("异步 mpsc channel：生产者-消费者");
    let (tx, mut rx) = mpsc::channel::<i32>(8);
    // 生产者任务。
    let producer = tokio::spawn(async move {
        for i in 0..5 {
            // send 是异步的：缓冲满时让出，而非阻塞。
            tx.send(i).await.unwrap();
            println!("    [生产] {i}");
            sleep(Duration::from_millis(5)).await;
        }
        // tx drop → 通道关闭。
    });
    // 消费者：主任务。
    while let Some(v) = rx.recv().await {
        println!("  [消费] {v}");
    }
    producer.await.unwrap();
    println!("  （通道关闭，recv 返回 None）");

    section("多个生产者：tx.clone()");
    let (tx, mut rx) = mpsc::channel(16);
    let mut producers = vec![];
    for id in 0..3 {
        let tx = tx.clone();
        producers.push(tokio::spawn(async move {
            for i in 0..3 {
                tx.send(format!("P{id}-msg{i}")).await.unwrap();
                sleep(Duration::from_millis(2)).await;
            }
        }));
    }
    drop(tx); // drop 原始 tx
    let mut msgs: Vec<String> = vec![];
    while let Some(m) = rx.recv().await {
        msgs.push(m);
    }
    msgs.sort();
    for m in &msgs {
        println!("  收到: {m}");
    }
    for p in producers {
        p.await.unwrap();
    }

    section("Stream：异步迭代器");
    // receiver 可以当 Stream 用（需要 futures crate 或 tokio_stream）。
    // 这里用「手动 recv 循环」模拟 Stream 迭代（避免额外依赖）。
    let (tx, mut rx) = mpsc::channel(8);
    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i * 10).await.unwrap();
        }
    });
    // while let Some 就是对 Stream 的「异步 for 循环」。
    let mut sum = 0;
    while let Some(v) = rx.recv().await {
        sum += v;
    }
    println!("  Stream 求和: {sum}");

    section("背压：channel 容量限制");
    // 容量为 2 → 生产者最多发 2 个就要等消费者取走。
    let (tx, mut rx) = mpsc::channel(2);
    let producer = tokio::spawn(async move {
        for i in 0..5 {
            // 满时 send 会 await（让出），这就是背压。
            tx.send(i).await.unwrap();
            println!("    [生产] {i}（已发出）");
        }
    });
    // 消费者慢慢取。
    while let Some(v) = rx.recv().await {
        println!("  [消费] {v}");
        sleep(Duration::from_millis(10)).await;
    }
    producer.await.unwrap();

    section("oneshot：一次性通道（发一个就关）");
    // 适合「请求-响应」：发一次结果，接收方等一次。
    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        sleep(Duration::from_millis(10)).await;
        tx.send("结果来了").unwrap();
    });
    let result: &str = rx.await.unwrap();
    println!("  oneshot 收到: {result}");

    section("broadcast：一个值广播给多个接收者");
    // 适合：事件通知、消息总线。
    let tx = tokio::sync::broadcast::channel::<&str>(16).0;
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();
    tokio::spawn(async move {
        sleep(Duration::from_millis(5)).await;
        tx.send("广播消息").unwrap();
    });
    // 两个订阅者都收到。
    let m1 = rx1.recv().await.unwrap();
    let m2 = rx2.recv().await.unwrap();
    println!("  订阅者1: {m1}");
    println!("  订阅者2: {m2}");

    section("watch：观察单个值的最新版本");
    // 适合：配置更新、状态广播（只关心最新值）。
    let (tx, mut rx) = tokio::sync::watch::channel(0);
    tokio::spawn(async move {
        for i in 1..=3 {
            sleep(Duration::from_millis(5)).await;
            tx.send(i);
        }
    });
    // watch 只看最新值（可能跳过中间值）。
    while rx.changed().await.is_ok() {
        println!("  watch 最新值: {}", *rx.borrow());
    }

    section("异步 channel vs 同步 channel（第 19 章）");
    println!("  std::sync::mpsc：send/recv 阻塞线程");
    println!("  tokio::sync::mpsc：send/recv 是 async（让出而非阻塞）");
    println!("  → 在 async 代码里必须用 tokio::sync::mpsc！");
    println!("  → 用同步 channel 会阻塞 reactor（致命）");
}
