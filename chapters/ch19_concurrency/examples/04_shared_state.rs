//! 19.4 共享可变状态：Mutex / RwLock / Arc
//!
//! 关键结论：
//! - `Mutex<T>`：互斥锁，同一时刻只允许一个线程访问 T。
//!   * `lock()` 返回 `MutexGuard`（RAII，离开作用域自动解锁）。
//!   * 内部可变性：`&self` 也能改 T。
//!   * 持锁线程 panic 会导致「中毒」（poisoned），后续 lock 返回 Err。
//! - `RwLock<T>`：读写锁，允许多个读或一个写。
//!   * `read()` 共享读锁；`write()` 独占写锁。
//!   * 读多写少的场景比 Mutex 高效。
//! - `Arc<T>`：原子引用计数的智能指针 —— 让多个线程「拥有」同一个值。
//!   * `Arc::clone(&arc)` 增加引用计数（不是深拷贝）。
//!   * Arc 单线程用 Rc 即可（Arc 有原子操作开销）。
//! - 组合 `Arc<Mutex<T>>`：多线程共享可变数据的标准配方。
//!
//! 运行：`cargo run -p ch19_concurrency --example 04_shared_state`

use ch19_concurrency::{BankAccount, section};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    section("Arc：原子引用计数，多线程共享");
    // Arc 让多个线程「共享」同一个只读数据。
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    for _ in 0..3 {
        let data = Arc::clone(&data); // 引用计数 +1（非深拷贝）
        handles.push(thread::spawn(move || {
            // 每个线程都能读到同一份数据。
            data.iter().sum::<i32>()
        }));
    }
    let sums: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("  各线程求和: {sums:?}（都看到同一份 data）");
    println!("  最终引用计数 = {}", Arc::strong_count(&data));

    section("Mutex：互斥锁，串行化访问");
    let counter = Arc::new(Mutex::new(0i32));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                // lock() 获取锁 → MutexGuard → 解引用就是 &mut i32。
                let mut num = counter.lock().unwrap();
                *num += 1;
                // num 离开作用域自动解锁。
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    // 10 线程 × 1000 次自增 = 10000，结果必须精确（锁保证）。
    println!("  最终 counter = {}（应为 10000）", *counter.lock().unwrap());

    section("Arc<Mutex<T>>：多线程共享可变数据的标准配方");
    let account = Arc::new(BankAccount::new(1000));
    let mut handles = vec![];
    // 10 个线程同时往账户存 100。
    for _ in 0..10 {
        let account = Arc::clone(&account);
        handles.push(thread::spawn(move || {
            account.deposit(100);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("  10 次存 100 后余额 = {}（应为 2000）", account.balance());

    section("Mutex 中毒（poisoned）：持锁线程 panic");
    let m = Arc::new(Mutex::new(0i32));
    let m2 = Arc::clone(&m);
    let h = thread::spawn(move || {
        let _guard = m2.lock().unwrap();
        panic!("持锁时 panic！"); // 锁会中毒
    });
    let _ = h.join();
    // 再次 lock 会返回 PoisonError。
    match m.lock() {
        Ok(_) => println!("  锁正常"),
        Err(e) => {
            println!("  锁中毒了: {e}");
            // 可以用 into_inner() 强行取出值（知道自己要承担风险）。
            let recovered = e.into_inner();
            println!("  强行取出: {recovered}");
        }
    }

    section("RwLock：读写锁，读多写少更高效");
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut reader_handles = vec![];
    // 多个读者并发读。
    for i in 0..3 {
        let data = Arc::clone(&data);
        reader_handles.push(thread::spawn(move || {
            let r = data.read().unwrap();
            println!("    读者 {i}: {:?}", *r);
            r.len()
        }));
    }
    // 一个写者（单独 handle，因为返回类型不同）。
    let writer_handle = {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut w = data.write().unwrap();
            w.push(4);
            println!("    写者: 添加元素后 {:?}", *w);
        })
    };
    for h in reader_handles {
        let _: usize = h.join().unwrap();
    }
    writer_handle.join().unwrap();

    section("RwLock 的「写者优先」与饥饿");
    // RwLock 不保证公平性：写者多时读者可能饥饿，反之亦然。
    // 实际：读 >> 写 时 RwLock 赢；读写均衡时 Mutex 反而更简单高效。
    println!("  读多写少 → RwLock；均衡或写多 → Mutex 更简单");

    section("避免死锁：锁的顺序");
    // 死锁：线程 A 持锁 1 等锁 2，线程 B 持锁 2 等锁 1。
    // 预防：所有线程「按固定顺序」获取多把锁。
    let lock1 = Arc::new(Mutex::new(()));
    let lock2 = Arc::new(Mutex::new(()));
    let (l1a, l2a) = (Arc::clone(&lock1), Arc::clone(&lock2));
    let (l1b, l2b) = (Arc::clone(&lock1), Arc::clone(&lock2));
    let h1 = thread::spawn(move || {
        // 固定顺序：先 1 后 2。
        let _g1 = l1a.lock().unwrap();
        let _g2 = l2a.lock().unwrap();
    });
    let h2 = thread::spawn(move || {
        // 同样顺序：先 1 后 2（若反过来就死锁）。
        let _g1 = l1b.lock().unwrap();
        let _g2 = l2b.lock().unwrap();
    });
    h1.join().unwrap();
    h2.join().unwrap();
    println!("  无死锁（两线程按相同顺序获取锁）");

    section("Arc vs Mutex 选择总结");
    println!("  只读共享 → Arc<T>");
    println!("  可变共享 → Arc<Mutex<T>>（或 Arc<RwLock<T>>）");
    println!("  单线程内部可变 → RefCell<T>（不能用 Arc）");
}
