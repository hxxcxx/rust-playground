//! 20.1 从同步到异步：为什么需要 async
//!
//! 关键结论：
//! - 「同步」IO 会阻塞整个线程 —— 一个连接一个线程，1 万连接要 1 万线程（OS 开销大）。
//! - 「异步」用单线程处理上万连接：IO 等待时不阻塞，转去处理其它就绪任务。
//! - 异步的本质：「等待时不占线程」—— 把等待时间让给其它任务。
//! - 协作式调度：异步任务在 `.await` 时「主动让出」，由运行时调度其它任务。
//! - 异步不是「更快」，而是「更省资源地处理大量并发」。
//!
//! 运行：`cargo run -p ch20_async --example 01_from_sync`

use ch20_async::section;
use std::thread;
use std::time::Duration;

fn main() {
    section("同步模型：一个任务占一个线程");
    // 同步 sleep 会阻塞整个线程 —— 这期间线程什么都做不了。
    println!("  [同步] 开始 3 个串行任务（每任务 100ms）...");
    let start = std::time::Instant::now();
    for i in 0..3 {
        sync_task(i);
    }
    println!("  [同步] 完成，总耗时 {:?}（串行 = 3 × 100ms）", start.elapsed());

    section("用多线程并行：每个任务一个线程");
    println!("  [线程] 启动 3 个线程（每任务 100ms）...");
    let start = std::time::Instant::now();
    let handles: Vec<_> = (0..3)
        .map(|i| thread::spawn(move || sync_task(i)))
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!("  [线程] 完成，总耗时 {:?}（并行 ≈ 100ms）", start.elapsed());
    println!("  （但每个连接都占一个 OS 线程 —— 1 万连接 = 1 万线程，开销大）");

    section("异步模型：单线程处理大量并发（用我们的 block_on 演示）");
    println!("  [异步] 用 block_on 跑一个 Delay Future...");
    let start = std::time::Instant::now();
    let result = ch20_async::block_on(async {
        println!("    [异步] 任务开始");
        ch20_async::delay(Duration::from_millis(50)).await;
        println!("    [异步] 任务完成");
        42
    });
    println!("  [异步] 返回值 = {result}, 耗时 {:?}", start.elapsed());

    section("异步的核心：等待时不占线程");
    // 这正是异步相对线程的优势：等待 IO 时，线程可以去 poll 其它 Future。
    // 而同步代码在 sleep 时，线程被 OS 挂起，无法做别的。
    println!("  同步 sleep：线程被挂起，无法做事");
    println!("  异步 .await：任务让出，线程可去 poll 其它任务");
    println!("  → 单线程 + 异步 = 能处理上万并发连接（如 Web 服务器）");

    section("异步 vs 多线程：何时用哪个？");
    println!("  ✅ 异步：大量 IO 密集型并发（网络服务、爬虫、数据库客户端）");
    println!("  ✅ 多线程：CPU 密集型并行计算（见第 19 章）");
    println!("  ✅ 混合：异步 + spawn_blocking（把 CPU 任务丢给线程池）");
    println!("  ❌ 异步不适合：纯 CPU 计算（无法在 .await 时让出）");

    section("「颜色」问题：函数有四种");
    // 著名的「函数着色」：async/await 让函数分成两类。
    println!("  同步函数 fn foo()              —— 红");
    println!("  异步函数 async fn foo()        —— 蓝");
    println!("  异步只能在异步里 .await；同步能在任何地方调用");
    println!("  → 异步代码会「传染」：调用异步 = 自己也得是异步");
    println!("  → Rust 用 spawn_blocking 在两类间搭桥");
}

/// 一个「同步」任务：sleep 模拟 IO 等待（阻塞线程）。
fn sync_task(id: usize) {
    thread::sleep(Duration::from_millis(100));
    println!("    [同步] 任务 {id} 完成");
}
