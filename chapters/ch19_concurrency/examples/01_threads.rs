//! 19.1 线程基础：spawn / join / move / panic 隔离
//!
//! 关键结论：
//! - `std::thread::spawn(closure)` 创建一个 OS 线程，返回 `JoinHandle<T>`。
//! - `handle.join()` 阻塞等待线程结束，返回 `Result<T, Box<dyn Any>>`：
//!   * Ok(v) —— 线程正常返回 v。
//!   * Err(e) —— 线程 panic 了，e 是 panic 值。
//! - 闭包默认「借用」捕获的变量 —— 但线程可能比借用活得久，所以必须 `move`。
//! - panic 隔离：一个线程 panic **不会**让整个进程崩溃（除非主线程 panic）。
//! - Rust 的「线程安全」靠类型系统：闭包必须是 `Send + 'static` 才能跨线程。
//!
//! 运行：`cargo run -p ch19_concurrency --example 01_threads`

use ch19_concurrency::section;
use std::thread;
use std::time::Duration;

fn main() {
    section("spawn + join：最朴素的线程");
    // spawn 返回 JoinHandle<u32>（闭包返回类型）。
    let handle = thread::spawn(|| {
        println!("    子线程: 工作中...");
        thread::sleep(Duration::from_millis(100));
        println!("    子线程: 完成");
        42 // 闭包的返回值，通过 join 取回
    });
    println!("  主线程: 等待子线程...");
    // join 阻塞直到子线程结束，返回 Result。
    let result = handle.join().unwrap();
    println!("  主线程: 子线程返回 {result}");

    section("move 关键字：让线程拥有捕获的数据");
    // 没有 move：编译失败 —— 闭包借用了 data，但线程可能比 data 活得久。
    // 加 move：闭包拿走 data 的所有权，线程可以放心使用。
    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("    子线程拿到 data: {data:?}");
        data.iter().sum::<i32>()
    });
    // data 已被 move 进线程，主线程不能再用。
    let sum = handle.join().unwrap();
    println!("  子线程求和 = {sum}");

    section("多个线程并发：顺序不确定");
    let mut handles = vec![];
    for i in 0..5 {
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            println!("    线程 {i} 完成");
            i * i
        });
        handles.push(handle);
    }
    // 收集所有结果。join 的顺序固定（按 handles 顺序），但「执行」是并发的。
    let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("  各线程平方: {results:?}");

    section("线程返回值 vs 主线程通信");
    // 方式 1：通过 join 取回返回值（适合一次性任务）。
    // 方式 2：通过 channel 通信（见 03_channels）。
    // 方式 3：通过 Arc<Mutex<T>> 共享（见 04_shared_state）。
    let h = thread::spawn(|| "来自线程的字符串".to_string());
    let msg: String = h.join().unwrap();
    println!("  join 取回: {msg}");

    section("panic 隔离：子线程 panic 不影响主线程");
    let handle = thread::spawn(|| {
        panic!("子线程故意 panic！");
    });
    // join 返回 Err —— panic 被隔离在线程内。
    let result = handle.join();
    match result {
        Ok(v) => println!("  成功: {v}"),
        Err(e) => println!("  捕获到线程 panic: {:?}", e.downcast_ref::<&str>()),
    }
    println!("  主线程继续运行（没崩溃）");

    section("线程必须 Send + 'static");
    // spawn 要求闭包: Send + 'static。
    // - Send：闭包捕获的所有数据都能跨线程转移。
    // - 'static：没有非静态借用（线程可能活得比调用栈久）。
    let x = String::from("hello"); // String 是 Send
    thread::spawn(move || {
        println!("    {x}（x 是 Send + 'static）");
    })
    .join()
    .unwrap();

    section("线程 ID：每个线程有唯一标识");
    let main_id = thread::current().id();
    let handle = thread::spawn(|| thread::current().id());
    let child_id = handle.join().unwrap();
    println!("  主线程 ID: {main_id:?}");
    println!("  子线程 ID: {child_id:?}");
    println!("  相同? {}", main_id == child_id);

    section("sleep：让出 CPU（演示并发交错）");
    let h1 = thread::spawn(|| {
        for _ in 0..3 {
            print!("A");
            thread::sleep(Duration::from_millis(1));
        }
    });
    let h2 = thread::spawn(|| {
        for _ in 0..3 {
            print!("B");
            thread::sleep(Duration::from_millis(1));
        }
    });
    h1.join().unwrap();
    h2.join().unwrap();
    println!("\n  （A 和 B 交错输出，说明并发执行）");
}
