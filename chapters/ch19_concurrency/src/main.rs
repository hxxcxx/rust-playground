//! 第19章 并发 —— 入口。
//!
//! 章节示例：
//! - `01_threads`        —— 线程基础：spawn / join / move / panic 隔离
//! - `02_scoped_threads` —— 作用域线程：thread::scope 安全借用栈数据
//! - `03_channels`       —— 通道：mpsc / sync_channel / 多生产者 / 管道
//! - `04_shared_state`   —— Mutex / RwLock / Arc 共享可变状态（银行账户）
//! - `05_atomics`        —— 原子操作 + Send / Sync 详解
//! - `06_parallel`       —— 分治并行：并行求和 / 归并排序 / 工作分发

fn main() {
    println!("第19章 并发");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch19_concurrency --example 01_threads");
    println!("  cargo run -p ch19_concurrency --example 02_scoped_threads");
    println!("  cargo run -p ch19_concurrency --example 03_channels");
    println!("  cargo run -p ch19_concurrency --example 04_shared_state");
    println!("  cargo run -p ch19_concurrency --example 05_atomics");
    println!("  cargo run -p ch19_concurrency --example 06_parallel");
}
