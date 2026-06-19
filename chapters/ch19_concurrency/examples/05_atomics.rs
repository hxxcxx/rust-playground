//! 19.5 原子操作 + Send / Sync 详解
//!
//! 关键结论：
//! - 原子类型（AtomicBool / AtomicI32 / AtomicUsize / AtomicPtr...）：
//!   * 提供不可分割的「读-改-写」操作，无需锁。
//!   * 比 Mutex 轻量（无系统调用、不阻塞），但只保护「单个值」。
//!   * 常用于：计数器、标志位、无锁数据结构的构建块。
//! - 内存序（Ordering）：
//!   * Relaxed —— 只保证原子，不保证顺序（最快，用于计数）。
//!   * Acquire / Release —— 建立 happens-before（读写配对）。
//!   * AcqRel —— 读用 Acquire、写用 Release。
//!   * SeqCst —— 顺序一致（最强，默认心理预期）。
//! - Send / Sync 是「自动派生」的 marker trait：
//!   * Send：T 的所有权可以安全转移到另一线程。
//!   * Sync：&T 可以安全地被多线程共享（等价于 &T: Send）。
//!   * 由「字段的 Send/Sync」自动推导（类似 Ord 的派生）。
//!   * 不满足时编译期报错 —— 数据竞争无法编译通过。
//!
//! 运行：`cargo run -p ch19_concurrency --example 05_atomics`

use ch19_concurrency::{Counter, section};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::cell::Cell;

fn main() {
    section("AtomicUsize：无锁计数器");
    let counter = AtomicUsize::new(0);
    let counter = Arc::new(counter);
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                // fetch_add 原子地「加并返回旧值」，无需锁。
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("  原子计数: {}（应为 10000）", counter.load(Ordering::Relaxed));

    section("用库的 Counter（封装的 AtomicU64）");
    let counter = Arc::new(Counter::new());
    let mut handles = vec![];
    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..500 {
                c.increment();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("  Counter.get() = {}（应为 2000）", counter.get());

    section("AtomicBool：线程间标志位");
    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = Arc::clone(&stop);
    let handle = thread::spawn(move || {
        let mut count = 0;
        // 自旋等待 stop 变 true（教学用；实际用 channel/condvar 更好）。
        while !stop_clone.load(Ordering::Relaxed) {
            count += 1;
            if count > 1_000_000 {
                break; // 防止无限循环（教学保护）
            }
        }
        println!("    工作线程在第 {count} 次检查时停止");
        count
    });
    // 主线程稍后发出停止信号。
    thread::sleep(std::time::Duration::from_millis(5));
    stop.store(true, Ordering::Relaxed);
    let n = handle.join().unwrap();
    println!("  发出停止信号，工作线程循环了 {n} 次");

    section("compare_exchange：CAS（比较并交换）");
    // 经典无锁原语：只有当前值 == 期望时才更新。
    let val = AtomicUsize::new(10);
    // 期望 10，更新为 20 —— 成功（返回旧值 10）。
    let r1 = val.compare_exchange(10, 20, Ordering::SeqCst, Ordering::SeqCst);
    println!("  CAS(10→20): {:?}", r1);
    // 期望 10（但现在是 20）—— 失败，返回当前值。
    let r2 = val.compare_exchange(10, 30, Ordering::SeqCst, Ordering::SeqCst);
    println!("  CAS(10→30) 当值已是 20: {:?}", r2);
    println!("  当前值: {}", val.load(Ordering::SeqCst));

    section("Ordering 内存序：Relaxed vs Acquire/Release");
    // Relaxed：快，但不保证「其它变量的写入」可见。
    // Acquire/Release：保证「释放前的写入」对「获取后」可见。
    let data = Arc::new(Mutex::new(0i32)); // 用 Mutex 保证 data 写入可见
    let ready = Arc::new(AtomicBool::new(false));
    let (d1, r1) = (Arc::clone(&data), Arc::clone(&ready));
    let producer = thread::spawn(move || {
        // 先写 data，再设 ready（Release 保证顺序）。
        *d1.lock().unwrap() = 42;
        r1.store(true, Ordering::Release);
    });
    let (d2, r2) = (Arc::clone(&data), Arc::clone(&ready));
    let consumer = thread::spawn(move || {
        // 自旋等 ready（Acquire 配对 Release）。
        while !r2.load(Ordering::Acquire) {
            std::hint::spin_loop();
        }
        // 此时保证看到 data == 42（happens-before）。
        *d2.lock().unwrap()
    });
    producer.join().unwrap();
    let val = consumer.join().unwrap();
    println!("  Acquire/Release 保证可见: data = {val}");

    section("Send：所有权可跨线程转移");
    // 几乎所有类型都是 Send（String/Vec/i32/Arc...）。
    let s = String::from("send me");
    thread::spawn(move || {
        println!("    跨线程: {s}");
    })
    .join()
    .unwrap();
    println!("  String 是 Send ✓");

    section("Rc 不是 Send：不能跨线程");
    // Rc 用非原子的引用计数 → 跨线程会数据竞争。
    // Rust 在编译期拒绝：
    let rc = std::rc::Rc::new(5);
    println!("  Rc 创建成功（单线程可用）");
    // 下面会编译失败（Rc: !Send）：
    // thread::spawn(move || { let _ = rc; });
    println!("  （Rc 是 !Send，跨线程需用 Arc）");

    section("RefCell 不是 Sync：不能跨线程共享 &RefCell");
    let cell = Cell::new(0); // Cell 也不是 Sync
    cell.set(42);
    println!("  Cell 单线程: {}", cell.get());
    // 下面会编译失败（Cell: !Sync）：
    // let c = &cell; thread::spawn(move || { c.get(); });
    println!("  （Cell/RefCell 是 !Sync，多线程需用 Mutex/Atomic）");

    section("Send / Sync 的自动推导");
    // struct MyType { a: i32, b: String } —— 自动是 Send + Sync（字段都是）。
    // struct WithRc(Rc<i32>) —— 自动是 !Send + !Sync（Rc 不是）。
    // struct WithRef(*const u8) —— 自动是 !Send + !Sync（裸指针不是）。
    #[derive(Default)]
    struct Safe { x: i32, y: String }
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    assert_send::<Safe>();
    assert_sync::<Safe>();
    println!("  Safe (i32 + String) 自动是 Send + Sync ✓");
}
