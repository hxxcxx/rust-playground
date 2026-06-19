//! 19.2 作用域线程：thread::scope 安全借用栈数据
//!
//! 关键结论：
//! - 普通 `spawn` 要求闭包 `'static` —— 不能借用栈上的局部变量。
//! - `thread::scope(|s| { s.spawn(|| ...); })` 解决这个限制：
//!   * scope 块结束时，会**自动 join** 所有内部 spawn 的线程。
//!   * 因此这些线程的存活期「不会超过 scope 块」。
//!   * 于是闭包可以安全地借用栈上变量（编译器能证明借用合法）。
//! - 用途：分治并行、对数组各部分并行处理、临时的工作线程组。
//! - scope 的返回值可以传出（只要不借用 scope 内的数据）。
//!
//! 运行：`cargo run -p ch19_concurrency --example 02_scoped_threads`

use ch19_concurrency::section;
use std::thread;

fn main() {
    section("scope 让线程借用栈数据");
    // data 是局部变量（栈上）。
    let data = vec![10, 20, 30, 40, 50];
    // 在 scope 内 spawn 的线程可以借用 &data。
    // scope 保证：所有线程在 scope 块结束前 join 完，data 不会提前销毁。
    thread::scope(|s| {
        s.spawn(|| {
            println!("    线程 1 读 data: {:?}", &data[0..3]);
        });
        s.spawn(|| {
            println!("    线程 2 读 data: {:?}", &data[3..5]);
        });
        // scope 块结束 → 自动 join 这两个线程。
    });
    // data 在 scope 后还能用（线程已结束，借用释放）。
    println!("  scope 后 data 仍可用: {data:?}");

    section("scope 内并行处理数组各段（分治）");
    let mut data = vec![0u64; 1_000_000];
    // 把数组的「可变切片」分给不同线程填充（不重叠 → 安全）。
    // 这是「分块并行」的经典模式。
    thread::scope(|s| {
        let chunk = data.len() / 4;
        for (i, part) in data.chunks_mut(chunk).enumerate() {
            s.spawn(move || {
                // 每个线程负责填充自己那段。
                for (j, slot) in part.iter_mut().enumerate() {
                    *slot = (i * chunk + j) as u64;
                }
            });
        }
    });
    println!("  并行填充后: data[0]={}, data[999999]={}", data[0], data[999999]);
    println!("  校验: sum = {}", data.iter().sum::<u64>());

    section("scope 收集子线程返回值");
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
    // 每个 scope 线程计算一段的和，主线程收集。
    let results: Vec<i32> = thread::scope(|s| {
        let mut handles = vec![];
        for chunk in numbers.chunks(2) {
            // chunk 是 &[i32]，借用 numbers —— scope 内合法。
            let h = s.spawn(move || chunk.iter().sum::<i32>());
            handles.push(h);
        }
        // scope 的返回值：收集各线程结果。
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    println!("  各段和: {results:?}, 总和 = {}", results.iter().sum::<i32>());

    section("scope 内可变借用：mut 并行修改");
    let mut counter = 0i32;
    thread::scope(|s| {
        // 注意：多个线程不能同时可变借用 counter（违反 Rust 规则）。
        // 这里是「顺序」spawn 但实际想并行 —— 会编译失败：
        // s.spawn(|| counter += 1);
        // s.spawn(|| counter += 1);  // ❌ 两次可变借用
        // 想并行修改同一数据 → 用 Mutex/Arc（见 04_shared_state）。
        // 这里演示单个线程的可变借用：
        s.spawn(|| {
            counter += 100;
        });
    });
    println!("  scope 后 counter = {counter}");

    section("scope 嵌套与异常安全");
    // 即使 scope 内某线程 panic，scope 也会等待所有线程，并传播 panic。
    let result = std::panic::catch_unwind(|| {
        thread::scope(|s| {
            s.spawn(|| panic!("scope 内 panic"));
        });
    });
    println!("  scope 内 panic 是否传播: {}", result.is_err());

    section("scope vs 普通 spawn 选择");
    println!("  需要借用局部数据 / 等待所有线程 → scope");
    println!("  需要长期运行的后台线程 / 不阻塞调用方 → 普通 spawn");
    println!("  （scope 是日常并行任务的首选 —— 安全、简单、零开销）");
}
