//! 20.3 async fn / async block / .await 基础
//!
//! 关键结论：
//! - `async fn foo() -> T` 定义一个异步函数，返回 `impl Future<Output = T>`。
//! - 调用 async fn **不执行**它，只是创建一个 Future（惰性）。
//! - `.await` 推动 Future 到完成（阻塞当前 async 上下文直到 Ready）。
//! - `async { ... }` 块创建匿名 Future，类似立即调用的 async fn。
//! - async fn 里可以 `?` 处理 Result（要求返回类型是 Result）。
//! - async fn 的「返回值」= Future 的 Output。
//!
//! 本示例用我们手写的 block_on 驱动（无运行时依赖），讲清 async/await 本质。
//!
//! 运行：`cargo run -p ch20_async --example 03_async_await`

use ch20_async::{block_on, delay, ready, section};
use std::time::Duration;

fn main() {
    section("async fn：定义异步函数");
    // async fn 调用返回 Future，不立即执行。
    let future = greet("Rust"); // 此刻 greet 还没跑
    println!("  创建了 Future（greet 还没执行）");
    block_on(future); // 现在 block_on 驱动它，greet 才执行

    section("async fn 的返回值");
    let future = add_async(3, 4);
    let result = block_on(future);
    println!("  add_async(3,4) = {result}");

    section(".await：在 async 上下文里等待");
    block_on(async {
        // .await 只能在 async 块/函数里用。
        let a = compute_async(10).await;
        let b = compute_async(20).await;
        println!("  串行 await: {} + {} = {}", a, b, a + b);
    });

    section("async 块：匿名 Future");
    let result = block_on(async {
        let x = ready(5).await;
        let y = ready(10).await;
        x * y
    });
    println!("  async 块结果: {result}");

    section("async 块捕获变量（move）");
    let multiplier = 3;
    let f = async move {
        let v = ready(7).await;
        v * multiplier // 捕获 multiplier
    };
    println!("  捕获 multiplier=3: {}", block_on(f));

    section("async fn 里的 ? 错误处理");
    match block_on(parse_and_double("21")) {
        Ok(n) => println!("  parse_and_double(\"21\") = {n}"),
        Err(e) => println!("  错误: {e}"),
    }
    match block_on(parse_and_double("abc")) {
        Ok(n) => println!("  {n}"),
        Err(e) => println!("  parse_and_double(\"abc\") 错误: {e}"),
    }

    section("async fn 嵌套调用");
    let result = block_on(process_async());
    println!("  process_async() = {result}");

    section("串行 await vs 并发 await");
    // 串行：一个 await 完才 await 下一个（总时间 = 之和）。
    let start = std::time::Instant::now();
    block_on(async {
        delay(Duration::from_millis(40)).await;
        delay(Duration::from_millis(40)).await;
    });
    println!("  串行 2×40ms: {:?}", start.elapsed());

    // 并发：用 async 块 + 我们的「同时驱动」（这里只是串行，真正的并发见 05_concurrency）。
    // 说明：手写 block_on 只能驱动一个 Future，要并发需要运行时（tokio::join!）。
    println!("  （要真正并发 → 需 tokio::join!，见 05_concurrency）");

    section("async fn 的类型：返回 impl Future");
    // async fn greet() 实际签名是 fn greet() -> impl Future<Output = ()>。
    // 调用 greet() 得到的是 Future，不是 ()。
    let _f1 = greet("a");
    let _f2 = greet("b");
    // _f1 和 _f2 都是 Future（还没执行）。
    println!("  两次调用 greet() 得到两个独立的 Future（惰性）");

    section("async fn 与闭包");
    // async 闭包（用 async 块模拟）。
    let make_async = |n: i32| async move { n * n };
    let r1 = block_on(make_async(5));
    let r2 = block_on(make_async(6));
    println!("  async 闭包: 5²={r1}, 6²={r2}");
}

/// async fn：打印问候。
async fn greet(name: &str) {
    println!("    你好, {name}！");
}

/// async fn 带返回值。
async fn add_async(a: i32, b: i32) -> i32 {
    a + b
}

/// async fn 调用其它 async fn（演示嵌套）。
async fn compute_async(n: i32) -> i32 {
    ready(n).await
}

/// async fn 里用 ? 处理错误。
async fn parse_and_double(s: &str) -> Result<i32, String> {
    // ? 在 async fn 里照常工作。
    let n: i32 = s.parse().map_err(|e| format!("解析失败: {e}"))?;
    Ok(n * 2)
}

/// 嵌套的 async fn：调用多个 async 函数。
async fn process_async() -> i32 {
    let a = add_async(1, 2).await;
    let b = add_async(3, 4).await;
    add_async(a, b).await
}
