//! 第19章 并发（Concurrency）—— 共享类型与并发安全工具。
//!
//! 本章核心：Rust 的并发模型，以及「用类型系统在编译期保证线程安全」。
//!
//! - 线程（threads）：`std::thread::spawn` 创建，`JoinHandle::join` 等待。
//! - 作用域线程（scoped threads）：`thread::scope` 让线程安全地「借用栈上数据」。
//! - 通道（channels）：`mpsc`（多生产者单消费者）传递所有权，避免共享。
//! - 共享可变状态：`Mutex<T>`（互斥锁）/ `RwLock<T>`（读写锁）+ `Arc<T>`（原子引用计数）。
//! - 原子操作：`AtomicUsize` / `AtomicBool` 等，无锁同步。
//! - `Send` / `Sync`：两个自动派生的 marker trait，是 Rust 线程安全的根基：
//!   * `Send`：类型可以「安全地跨线程转移所有权」。
//!   * `Sync`：类型可以「被多个线程安全地共享引用」（&T 是 Send）。
//!
//! Rust 的并发哲学：把「数据竞争」变成编译期错误 —— 如果代码能编译，
//! （在 unsafe 之外）就不会有数据竞争。

use std::sync::{Mutex, atomic::AtomicU64};

// =======================================================================
// 工具函数
// =======================================================================

/// 打印带标题的分割线（与前几章风格一致）。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

// =======================================================================
// 例：一个线程安全的银行账户（Mutex 保护余额）
// =======================================================================

/// 银行账户：用 `Mutex<i64>` 保护余额，确保并发转账不会丢钱。
///
/// 这是「共享可变状态」的经典例子：
/// - 不加锁：两个线程同时读到旧值、各自写回 → 丢失更新。
/// - 加锁：同一时刻只有一个线程能修改余额。
///
/// 注意：内部可变性（`&self` 即可修改 balance）—— Mutex 提供。
pub struct BankAccount {
    // Mutex 提供「内部可变性」+「线程安全」。
    balance: Mutex<i64>,
}

impl BankAccount {
    /// 开户，初始余额为 0。
    pub fn new(initial: i64) -> Self {
        Self {
            balance: Mutex::new(initial),
        }
    }

    /// 存入金额。返回新余额。
    pub fn deposit(&self, amount: i64) -> i64 {
        // lock() 返回 MutexGuard，离开作用域自动解锁（RAII）。
        // lock() 可能「中毒」（poisoned）：持锁线程 panic 过 —— 这里用 unwrap。
        let mut bal = self.balance.lock().unwrap();
        *bal += amount;
        *bal
    }

    /// 取款；余额不足返回 Err。
    pub fn withdraw(&self, amount: i64) -> Result<i64, &'static str> {
        let mut bal = self.balance.lock().unwrap();
        if *bal < amount {
            return Err("余额不足");
        }
        *bal -= amount;
        Ok(*bal)
    }

    /// 查询当前余额（只读，也要加锁保证看到一致值）。
    pub fn balance(&self) -> i64 {
        *self.balance.lock().unwrap()
    }
}

// =======================================================================
// 例：原子计数器（无锁）
// =======================================================================

/// 用 AtomicU64 实现的计数器：无锁、比 Mutex 更轻量。
///
/// 适用：简单的数值累计、ID 生成器、统计计数。
/// 不适用：需要「读-改-写」涉及多个字段的一致性（那要用 Mutex）。
pub struct Counter {
    value: AtomicU64,
}

impl Counter {
    pub const fn new() -> Self {
        Self {
            value: AtomicU64::new(0),
        }
    }

    /// 原子自增，返回自增「前」的旧值。
    pub fn fetch_add(&self, n: u64) -> u64 {
        // Ordering 决定内存可见性保证：
        // - Relaxed：只保证原子，不保证与其它变量的顺序（最快）。
        // - Acquire/Release：建立 happens-before 关系（常用）。
        // - SeqCst：顺序一致（最强、最慢）。
        self.value.fetch_add(n, std::sync::atomic::Ordering::Relaxed)
    }

    /// 原子自增，返回自增「后」的新值。
    pub fn increment(&self) -> u64 {
        self.fetch_add(1) + 1
    }

    /// 当前值。
    pub fn get(&self) -> u64 {
        self.value.load(std::sync::atomic::Ordering::Relaxed)
    }
}

// 为 Counter 提供 Default（clippy 建议：有 new() 默认实现时配套 Default）。
impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

// =======================================================================
// 例：并行归并排序的「合并」工具函数
// =======================================================================

/// 把两个已排序的切片合并成一个有序 Vec（归并排序的核心）。
///
/// 这是「分治并行」的基础：把数据切成两半，各自排序（可并行），再合并。
pub fn merge_sorted<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }
    // 把剩余部分追加（其中一个切片必然已耗尽）。
    result.extend(left[i..].iter().cloned());
    result.extend(right[j..].iter().cloned());
    result
}
