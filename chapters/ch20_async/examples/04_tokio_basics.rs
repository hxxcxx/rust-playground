//! 20.4 Tokio 运行时 / #[tokio::main] / spawn
//!
//! 关键结论：
//! - Tokio 是 Rust 事实标准的异步运行时（executor + reactor）：
//!   * executor：调度并 poll Future。
//!   * reactor：用 OS 的 IO 多路复用（epoll/kqueue/IOCP）监听 IO 就绪。
//! - `#[tokio::main]`：把 main 包成一个多线程运行时，自己作为 async 入口。
//! - `tokio::spawn(future)`：把 Future 丢给运行时「后台」执行（类似线程，但极轻量）。
//! - `tokio::time::sleep`：真正的异步定时器（不阻塞线程，到点被 reactor 唤醒）。
//! - spawn 返回 JoinHandle，可 await 取回结果。
//!
//! 运行：`cargo run -p ch20_async --example 04_tokio_basics`

use ch20_async::section;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    section("#[tokio::main]：异步入口");
    println!("  运行在 Tokio 多线程运行时上");
    // 现在我们在 async 上下文里，可以 .await。
    let n = fetch_data().await;
    println!("  fetch_data() = {n}");

    section("tokio::time::sleep：真正的异步定时器");
    // sleep 不阻塞线程！它注册一个定时器，到点被 reactor 唤醒。
    let start = std::time::Instant::now();
    sleep(Duration::from_millis(50)).await;
    println!("  sleep(50ms) 等了 {:?}", start.elapsed());

    section("tokio::spawn：后台并发任务");
    // spawn 把 Future 丢给运行时，立即返回 JoinHandle。
    let handle = tokio::spawn(async {
        println!("    [spawn 任务] 开始");
        sleep(Duration::from_millis(20)).await;
        println!("    [spawn 任务] 完成");
        "结果"
    });
    println!("  spawn 返回，主任务继续...");
    // .await 等待 spawn 的任务完成，取回返回值。
    let result = handle.await.unwrap();
    println!("  spawn 任务返回: {result:?}");

    section("并发：多个 spawn 同时跑（单线程也能并发）");
    let start = std::time::Instant::now();
    let mut handles = vec![];
    for i in 0..5 {
        // 每个 task sleep 30ms —— 并发的话总共约 30ms。
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(30)).await;
            i * i
        });
        handles.push(handle);
    }
    let mut results = vec![];
    for h in handles {
        results.push(h.await.unwrap());
    }
    println!("  5 个并发任务（各 30ms）总耗时 {:?}（并行 ≈ 30ms）", start.elapsed());
    println!("  结果: {results:?}");

    section("tokio::join!：并发等待多个 Future（宏）");
    let start = std::time::Instant::now();
    // join! 让多个 Future 并发执行，全部完成后返回元组。
    let (a, b, c) = tokio::join!(task_a(), task_b(), task_c());
    println!("  join! 结果: {a}, {b}, {c}");
    println!("  3 个并发任务（各 40ms）总耗时 {:?}", start.elapsed());

    section("串行 await vs 并发 join!：耗时对比");
    // 串行：3 × 40ms = 120ms
    let start = std::time::Instant::now();
    let _ = task_a().await;
    let _ = task_b().await;
    let _ = task_c().await;
    println!("  串行 3×40ms: {:?}", start.elapsed());

    // 并发：≈ 40ms
    let start = std::time::Instant::now();
    let _ = tokio::join!(task_a(), task_b(), task_c());
    println!("  join! 并发: {:?}", start.elapsed());

    section("运行时类型：current_thread vs multi_thread");
    // #[tokio::main] 默认 multi_thread（多线程，用所有 CPU 核）。
    // 也可以指定单线程：#[tokio::main(flavor = "current_thread")]。
    // 单线程也支持并发 —— 因为并发 ≠ 并行。
    println!("  multi_thread：多核并行 + 单核并发");
    println!("  current_thread：仅单核并发（更省资源，适合轻量任务）");
    println!("  并发（concurrency）≠ 并行（parallelism）：异步主要解决并发");

    section("spawn 的任务 panic：隔离");
    let handle = tokio::spawn(async {
        panic!("spawn 任务 panic！");
    });
    // JoinError 表示任务 panic（或被取消）。
    let result = handle.await;
    println!("  spawn 任务结果: {:?}", result.is_err().then_some("panic 被隔离"));

    section("tokio 生态：标准库只给 trait，运行时靠生态");
    println!("  tokio：运行时 + IO + 定时器 + 同步原语");
    println!("  hyper：HTTP（基于 tokio）");
    println!("  reqwest：HTTP 客户端（基于 hyper）");
    println!("  tonic：gRPC");
    println!("  sqlx：异步数据库");
}

/// 模拟「获取数据」的异步函数。
async fn fetch_data() -> i32 {
    sleep(Duration::from_millis(10)).await;
    42
}

async fn task_a() -> &'static str {
    sleep(Duration::from_millis(40)).await;
    "A"
}

async fn task_b() -> &'static str {
    sleep(Duration::from_millis(40)).await;
    "B"
}

async fn task_c() -> &'static str {
    sleep(Duration::from_millis(40)).await;
    "C"
}
