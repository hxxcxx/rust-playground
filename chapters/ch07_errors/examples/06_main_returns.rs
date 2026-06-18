//! 7.6 main() 中处理错误
//!
//! 关键结论：
//! - 错误传到 main 后就「无处可传」了 —— 必须在 main 中处理。
//! - 三种典型策略：
//!   1. `main() -> Result<(), E>`：返回 Result，失败时打印 Error: ... 然后退出码 1。
//!   2. `.expect("msg")`：失败 panic（适合写小工具/原型，但错误信息有点吓人）。
//!   3. 手动 if let + print_error + process::exit(1)：完全控制输出格式。
//! - `main() -> Result` 要求错误类型实现 `Debug`，标准库错误类型都满足。
//!
//! 运行：`cargo run -p ch07_errors --example 06_main_returns`

use ch07_errors::{AppError, print_error, section};
use std::fs;

fn main() {
    section("策略 1：main() -> Result<(), E>");
    // 这个 example 的 main 本身不返回 Result，但我们可以调用一个返回 Result 的函数
    if let Err(e) = inner_main_strategy_result() {
        // eprintln!("Error: {e:?}"); ← 这是策略 1 的等价输出
        println!("  策略 1 错误: {e}");
    }

    section("策略 2：expect(msg) —— panic 式");
    let _ok = strategy_expect(); // 成功路径才到这里
    println!("  策略 2 成功（失败时会 panic）");

    section("策略 3：手动 if let + print_error + exit(1)");
    strategy_manual();

    section("真实生产代码：返回 Result 的 main");
    println!("  fn main() -> Result<(), Box<dyn Error>> {{");
    println!("      let config = load_config()?;");
    println!("      run(config)?;");
    println!("      Ok(())");
    println!("  }}");
}

/// 模拟策略 1：函数返回 Result，main 用 if let 处理
fn inner_main_strategy_result() -> Result<(), AppError> {
    // 故意读一个不存在的文件以触发 io::Error
    let path = std::env::temp_dir().join("ch07_not_exist_xxx.txt");
    fs::read_to_string(&path).map(|_| ())?;
    Ok(())
}

/// 策略 2：用 expect —— 失败就 panic
fn strategy_expect() -> String {
    // 给一个一定成功的场景做演示
    let content = "hello".to_string();
    // 如果是 fs::read_to_string(&path).expect("读取失败");
    // 失败时会 panic，打印类似：
    // thread 'main' panicked at '读取失败: ...'
    content
}

/// 策略 3：手动控制错误输出
fn strategy_manual() {
    let path = std::env::temp_dir().join("ch07_not_exist_yyy.txt");
    if let Err(err) = fs::read_to_string(&path) {
        print_error(&err);
        // std::process::exit(1); // 真实代码会退出；这里只是演示
        println!("  (这里会调用 std::process::exit(1))");
    } else {
        println!("  (读到文件)");
    }
}
