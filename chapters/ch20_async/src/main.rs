//! 第20章 异步编程 —— 入口。
//!
//! 章节示例：
//! - `01_from_sync`     —— 从同步到异步：为什么需要 async（线程 vs 异步）
//! - `02_future_trait`  —— Future trait / poll / Poll / Pin / 手写 Future
//! - `03_async_await`   —— async fn / async block / .await 基础（手写执行器）
//! - `04_tokio_basics`  —— Tokio 运行时 / #[tokio::main] / spawn
//! - `05_concurrency`   —— join!/select!/race + 并发组合 + spawn_blocking
//! - `06_streams`       —— Stream / 异步迭代器 + 异步 channel

fn main() {
    println!("第20章 异步编程");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch20_async --example 01_from_sync");
    println!("  cargo run -p ch20_async --example 02_future_trait");
    println!("  cargo run -p ch20_async --example 03_async_await");
    println!("  cargo run -p ch20_async --example 04_tokio_basics");
    println!("  cargo run -p ch20_async --example 05_concurrency");
    println!("  cargo run -p ch20_async --example 06_streams");
}
