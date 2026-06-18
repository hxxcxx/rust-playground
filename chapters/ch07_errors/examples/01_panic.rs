//! 7.1 panic：程序员的失误
//!
//! 关键结论：
//! - panic 用于「不该发生」的错误：数组越界、除零、`Option::unwrap` on None、断言失败。
//! - 默认行为是「展开栈」：逐层 drop 局部变量，类似 C++ 异常。
//! - `catch_unwind` 可捕获 panic 让线程继续运行（测试框架就是用它的）。
//! - panic 是「安全的」—— 不会留下悬空指针或半初始化值。
//! - 编译时用 `-C panic=abort` 可改成「直接终止」，缩小二进制体积。
//!
//! 运行：`cargo run -p ch07_errors --example 01_panic`

use ch07_errors::section;
use std::panic;

fn main() {
    section("除零触发 panic");
    // 这行不能直接写 —— 程序会真的崩溃退出。
    // 我们用 catch_unwind 捕获，演示「panic 是可以拦截的」。
    let result = panic::catch_unwind(|| {
        let _ = 10_i32 / 0; // ← panic: attempt to divide by zero
    });
    println!(
        "  catch_unwind 结果: {:?}",
        result.as_ref().map(|_| "正常").err()
    );

    section("显式 panic!() 宏");
    let result = panic::catch_unwind(|| {
        panic!("自定义错误消息: 数据损坏 {}", 42);
    });
    let payload = result.as_ref().err();
    println!("  panic! 被拦截: payload = {:?}", payload.is_some());

    section("数组越界触发 panic");
    // 用变量代替字面索引，避免 clippy 静态检测出越界
    let idx = 10;
    let v = [1, 2, 3];
    let result = panic::catch_unwind(|| {
        let _ = v[idx]; // 越界 → panic
    });
    println!("  越界访问结果: 已被拦截 = {}", result.is_err());

    section("断言失败触发 panic");
    let result = panic::catch_unwind(|| {
        assert_eq!(1, 2, "1 应该等于 2（不可能）");
    });
    println!("  assert_eq! 失败: 已被拦截 = {}", result.is_err());

    section("Option / Result 的 unwrap / expect 触发 panic");
    // 用变量代替字面量 None，避免 clippy 把它当成「literal unwrap」静态求值
    let some: Option<i32> = if std::env::args().count() > 9999 {
        Some(0)
    } else {
        None
    };
    let result = panic::catch_unwind(|| {
        let _ = some.expect("应该有值啊");
    });
    println!("  expect on None: 已被拦截 = {}", result.is_err());

    section("展开栈 vs 终止：Rust 给你选择");
    println!("  默认：unwind（展开栈）—— 每层函数的临时值按相反顺序 drop");
    println!("  可选：abort（编译时 -C panic=abort）—— 二进制更小但无法恢复");
    println!("  嵌套 panic（drop 中再次 panic）→ 强制 abort");
}

/// 模拟书中的海盗分赃：船长拿走一半，剩余除以船员数。
/// 船员为 0 时会触发 panic。
#[allow(dead_code)]
fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}
