//! 20.5 并发组合：join! / select! / race + spawn_blocking
//!
//! 关键结论：
//! - `tokio::join!(a, b, c)`：并发执行多个 Future，**全部完成**才返回（「会合点」）。
//!   适合：需要多个结果都拿到（如并行调用多个服务）。
//! - `tokio::select! { a => ..., b => ... }`：多个 Future「先到先得」，完成一个就退出。
//!   适合：超时、多路复用、取消。
//! - `tokio::time::timeout`：给 Future 加超时（基于 select!）。
//! - `spawn_blocking`：把「阻塞/重 CPU」任务丢到专用线程池，不阻塞 reactor。
//!   规则：async 里绝不调用同步阻塞 IO（std::thread::sleep / 同步文件 / 同步网络）！
//!
//! 运行：`cargo run -p ch20_async --example 05_concurrency`

use ch20_async::section;
use std::time::Duration;
use tokio::time::{sleep, timeout};

#[tokio::main]
async fn main() {
    section("tokio::join!：全部完成（会合点）");
    // 三个 task 各 30ms，join! 并发 → 总共约 30ms。
    let start = std::time::Instant::now();
    let (a, b, c) = tokio::join!(slow_op("A", 30), slow_op("B", 30), slow_op("C", 30));
    println!("  join! 结果: {a}, {b}, {c}（耗时 {:?}）", start.elapsed());

    section("tokio::try_join!：任一失败立即返回");
    // 三个 task，第二个失败 → try_join! 立即返回 Err。
    let result = tokio::try_join!(ok_op("A"), fail_op("B"), ok_op("C"));
    println!("  try_join! 结果: {:?}", result);

    section("tokio::select!：先到先得");
    // 两个 Future，谁先完成谁被处理，另一个被 drop（取消）。
    let result = tokio::select! {
        r1 = slow_op("快", 10) => {
            println!("    快任务赢了: {r1}");
            r1
        }
        r2 = slow_op("慢", 100) => {
            println!("    慢任务赢了: {r2}");
            r2
        }
    };
    println!("  select! 结果: {result}");

    section("select! 取消未完成的 Future");
    // select! 会 drop 没赢的 Future —— 这是「取消」机制。
    tokio::select! {
        _ = sleep(Duration::from_millis(50)) => println!("  定时器先到"),
        _ = never_completes() => println!("  永不完成的任务赢了（不可能）"),
    }
    println!("  never_completes 被取消（drop）");

    section("timeout：给 Future 加超时");
    // timeout(20ms, slow_op(100ms)) —— 100ms 的任务被 20ms 超时打断。
    let result = timeout(Duration::from_millis(20), slow_op("慢任务", 100)).await;
    match result {
        Ok(v) => println!("  任务完成: {v}"),
        Err(_) => println!("  超时！任务被取消"),
    }

    section("timeout：任务在时限内完成");
    let result = timeout(Duration::from_millis(50), slow_op("快任务", 10)).await;
    println!("  结果: {:?}", result);

    section("select! 循环：事件循环模式");
    // 经典模式：用 select! 在多个事件源间循环分发。
    let (tx1, mut rx1) = tokio::sync::mpsc::channel::<&str>(10);
    let (tx2, mut rx2) = tokio::sync::mpsc::channel::<&str>(10);
    tokio::spawn(async move {
        tx1.send("来自通道1").await.unwrap();
        sleep(Duration::from_millis(5)).await;
        tx1.send("通道1第二条").await.unwrap();
    });
    tokio::spawn(async move {
        sleep(Duration::from_millis(2)).await;
        tx2.send("来自通道2").await.unwrap();
    });
    let mut got = 0;
    while got < 3 {
        tokio::select! {
            Some(msg) = rx1.recv() => {
                println!("    [通道1] {msg}");
                got += 1;
            }
            Some(msg) = rx2.recv() => {
                println!("    [通道2] {msg}");
                got += 1;
            }
        }
    }

    section("spawn_blocking：把阻塞任务丢给线程池");
    // async 里调用同步阻塞会卡住 reactor（影响所有任务）！
    // spawn_blocking 把它丢到专用阻塞线程池，返回可 await 的 handle。
    let heavy = tokio::task::spawn_blocking(|| {
        // 这里是同步代码：可以调用 std::thread::sleep、同步 IO、CPU 计算。
        std::thread::sleep(Duration::from_millis(30));
        (0..1000).map(|i| i * i).sum::<i64>()
    });
    println!("  spawn_blocking 提交，主任务不阻塞...");
    // 同时可以做别的异步工作。
    sleep(Duration::from_millis(10)).await;
    let sum = heavy.await.unwrap();
    println!("  spawn_blocking 结果: {sum}");

    section("规则：async 里禁用同步阻塞");
    // ❌ 错误（卡 reactor）：
    //   std::thread::sleep(...);  // 阻塞整个 worker 线程
    //   std::fs::read(...);       // 同步文件 IO
    //   std::net::TcpStream...    // 同步网络
    // ✅ 正确：
    //   tokio::time::sleep(...).await;  // 异步定时器
    //   tokio::fs::read(...).await;     // 异步文件 IO
    //   tokio::net::TcpStream...        // 异步网络
    //   tokio::task::spawn_blocking(...) // 把同步阻塞丢给线程池
    println!("  async 里只能用 async 版本的 IO/定时器");
    println!("  必须用同步代码 → spawn_blocking 包起来");

    section("race!：多个 Future 取最先完成的");
    // tokio::race! (或 futures::race) 返回最先完成的结果。
    // 这里用 select! 模拟（select! 取第一个，但不收集结果）。
    let winner = tokio::select! {
        r = slow_op("选手1", 20) => r,
        r = slow_op("选手2", 10) => r,
    };
    println!("  race 赢家: {winner}（选手2，10ms < 20ms）");
}

/// 模拟耗时异步操作。
async fn slow_op(name: &'static str, ms: u64) -> &'static str {
    sleep(Duration::from_millis(ms)).await;
    name
}

async fn ok_op(name: &'static str) -> Result<&'static str, &'static str> {
    sleep(Duration::from_millis(10)).await;
    Ok(name)
}

async fn fail_op(_name: &'static str) -> Result<&'static str, &'static str> {
    sleep(Duration::from_millis(15)).await;
    Err("故意失败")
}

/// 一个永不完成的 Future（演示取消）。
async fn never_completes() -> i32 {
    // 用一个超长 sleep 模拟「永不完成」。
    sleep(Duration::from_secs(3600)).await;
    0
}
