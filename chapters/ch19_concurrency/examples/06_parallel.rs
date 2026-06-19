//! 19.6 分治并行：并行求和 / 归并排序 / 工作分发
//!
//! 关键结论：
//! - 「分治并行」：把大任务切成小块，分给多个线程，最后合并结果。
//! - 关键决策：「何时不再细分」—— 任务太细时，线程创建/同步开销 > 收益。
//!   经验：每个线程至少处理几万~几十万元素，才值得并行。
//! - 三种典型模式：
//!   1. 并行 reduce（求和/求积/统计）—— 各算各的，最后合并。
//!   2. 并行分治（归并排序）—— 递归切分，排序后合并。
//!   3. 工作队列（生产者/消费者）—— 用 channel 分发任务。
//! - 工具：thread::scope（借用栈数据）+ chunk 分块 + 合并函数。
//!
//! 运行：`cargo run -p ch19_concurrency --example 06_parallel`

use ch19_concurrency::{merge_sorted, section};
use std::sync::mpsc;
use std::thread;

fn main() {
    section("并行求和：分块 reduce");
    let data: Vec<u64> = (1..=10_000_000).collect();
    let n_threads = 4;
    // 串行基准。
    let serial = data.iter().copied().sum::<u64>();
    // 并行：每线程算一段的和，最后相加。
    let parallel = parallel_sum(&data, n_threads);
    println!("  串行求和 = {serial}");
    println!("  并行求和（{n_threads} 线程）= {parallel}");
    println!("  结果一致: {}", serial == parallel);

    section("并行求和：性能对比");
    let t = std::time::Instant::now();
    let _ = data.iter().copied().sum::<u64>();
    let serial_time = t.elapsed();
    let t = std::time::Instant::now();
    let _ = parallel_sum(&data, n_threads);
    let parallel_time = t.elapsed();
    println!("  串行: {serial_time:?}");
    println!("  并行: {parallel_time:?}");
    println!("  （数据小时并行可能更慢 —— 线程创建有开销）");

    section("并行归并排序：分治递归");
    let mut data: Vec<i32> = (0..1000).rev().collect(); // 逆序
    parallel_merge_sort(&mut data, 4);
    println!("  排序后前 5: {:?}", &data[..5]);
    println!("  排序后后 5: {:?}", &data[data.len() - 5..]);
    // 验证确实有序。
    let is_sorted = data.windows(2).all(|w| w[0] <= w[1]);
    println!("  验证有序: {is_sorted}");

    section("工作队列：channel 分发任务");
    // 经典「生产者-消费者」：一个分发器发任务，多个 worker 抢任务。
    let n_tasks = 10;
    let n_workers = 3;
    let (task_tx, task_rx) = mpsc::channel();
    let task_rx = std::sync::Arc::new(std::sync::Mutex::new(task_rx));

    // 启动 worker 池。
    let mut workers = vec![];
    for id in 0..n_workers {
        let rx = std::sync::Arc::clone(&task_rx);
        workers.push(thread::spawn(move || {
            let mut done = 0;
            // 循环抢任务：lock 取出一个任务，立刻解锁（缩短临界区）。
            loop {
                let task = {
                    let guard = rx.lock().unwrap();
                    guard.try_recv()
                };
                match task {
                    Ok(n) => {
                        let sq = n * n;
                        done += 1;
                        println!("    worker {id}: 任务 {n}² = {sq}");
                    }
                    Err(mpsc::TryRecvError::Empty) => continue,
                    Err(mpsc::TryRecvError::Disconnected) => break,
                }
            }
            done
        }));
    }

    // 分发任务。
    for n in 1..=n_tasks {
        task_tx.send(n).unwrap();
    }
    drop(task_tx); // 关闭通道，触发 worker 退出。

    let mut total_done = 0;
    for w in workers {
        total_done += w.join().unwrap();
    }
    println!("  共完成 {total_done} 个任务（应为 {n_tasks}）");

    section("并行 map：对每个元素并行计算");
    let input: Vec<i32> = (0..100).collect();
    let output = parallel_map(&input, 4, |x| x * x);
    println!("  前 5 平方: {:?}", &output[..5]);
    println!("  校验: output[50] = {}（应为 2500）", output[50]);

    section("并行决策：什么时候值得并行？");
    println!("  ✅ 数据量大（>10万元素）且每元素计算重");
    println!("  ✅ 任务相互独立（无共享依赖）");
    println!("  ❌ 数据小 / 任务有顺序依赖 / IO 密集（线程帮不上忙）");
    println!("  ❌ 任务极细（线程创建开销 > 计算）");
    println!("  经验：先 profile 找瓶颈，再考虑并行；线程数 ≈ CPU 核数。");
}

/// 并行求和：把数据分 n_threads 段，各线程求和后合并。
fn parallel_sum(data: &[u64], n_threads: usize) -> u64 {
    // 数据量太小 → 直接串行（避免线程开销）。
    if data.len() < 10_000 || n_threads <= 1 {
        return data.iter().copied().sum();
    }
    // scope 内 spawn 线程，安全借用 data。
    let chunk = data.len().div_ceil(n_threads); // 向上取整
    std::thread::scope(|s| {
        let handles: Vec<_> = data
            .chunks(chunk)
            .map(|part| {
                s.spawn(move || part.iter().copied().sum::<u64>())
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    })
}

/// 并行归并排序：递归切分，排序后用 merge_sorted 合并。
fn parallel_merge_sort(data: &mut Vec<i32>, depth: u32) {
    let len = data.len();
    if len <= 1 {
        return;
    }
    // depth 用完 → 退化为串行排序（停止并行切分）。
    if depth == 0 || len < 1000 {
        data.sort();
        return;
    }
    // 切两半。
    let mid = len / 2;
    let mut left: Vec<i32> = data[..mid].to_vec();
    let mut right: Vec<i32> = data[mid..].to_vec();
    // 并行排两半（scope 借用 &mut left / &mut right —— 不重叠）。
    std::thread::scope(|s| {
        s.spawn(|| parallel_merge_sort(&mut left, depth - 1));
        s.spawn(|| parallel_merge_sort(&mut right, depth - 1));
    });
    // 合并两半到 data。
    let merged = merge_sorted(&left, &right);
    *data = merged;
}

/// 并行 map：把输入分块，各线程对各自块应用 f，最后拼接。
fn parallel_map<T, U, F>(input: &[T], n_threads: usize, f: F) -> Vec<U>
where
    T: Sync,
    U: Send,
    F: Fn(&T) -> U + Sync,
{
    if input.len() < 10_000 || n_threads <= 1 {
        return input.iter().map(&f).collect();
    }
    let chunk = input.len().div_ceil(n_threads);
    // f 必须是 Sync（被多线程共享引用）—— 用引用而非 move。
    let f = &f;
    std::thread::scope(|s| {
        let handles: Vec<_> = input
            .chunks(chunk)
            .map(|part| {
                s.spawn(move || part.iter().map(f).collect::<Vec<U>>())
            })
            .collect();
        let mut out = Vec::with_capacity(input.len());
        for h in handles {
            out.extend(h.join().unwrap());
        }
        out
    })
}
