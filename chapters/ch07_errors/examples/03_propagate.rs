//! 7.3 错误传播：`?` 运算符
//!
//! 关键结论：
//! - `expr?`：成功则取出值，失败则 `return Err(e.into())`。
//! - `?` 自动调用 `From::from` 做错误类型转换 —— 这是它能跨多种错误类型的关键。
//! - `?` 也可用于 `Option<T>`：在返回 `Option` 的函数里，`None` 提前返回。
//! - 在旧代码里可能看到 `try!()` 宏 —— `?` 的前身。
//! - `?` 只能用在返回 `Result` 或 `Option` 的函数里。
//!
//! 运行：`cargo run -p ch07_errors --example 03_propagate`

use ch07_errors::section;
use std::fs;
use std::io;
use std::path::Path;

/// 模拟书中的 move_all：把 src 目录下所有文件移动到 dst。
/// 几乎每行都 `?` —— 系统编程的常态。
fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        // 打开目录可能失败
        let entry = entry_result?; // 读取目录项可能失败
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?; // 重命名可能失败
    }
    Ok(()) // 终于完成
}

/// 不用 `?` 的等价写法 —— 非常冗长
#[allow(clippy::question_mark, reason = "演示不用 ? 的等价写法")]
fn move_all_verbose(src: &Path, dst: &Path) -> io::Result<()> {
    let dir = match src.read_dir() {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    for entry_result in dir {
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => return Err(e),
        };
        let dst_file = dst.join(entry.file_name());
        if let Err(e) = fs::rename(entry.path(), dst_file) {
            return Err(e);
        }
    }
    Ok(())
}

/// `?` 用于 Option：在返回 Option 的函数中，None 会提前返回。
fn first_word_len(s: &str) -> Option<usize> {
    let first = s.split_whitespace().next()?; // None 时提前返回
    Some(first.len())
}

fn main() {
    section("? 让错误向上传播");

    // 创建一个临时场景：源目录存在但目标目录不存在 → 演示错误传播
    let temp = std::env::temp_dir().join("ch07_propagate_demo");
    let _ = fs::create_dir_all(&temp);
    let src = temp.join("nonexistent_src");
    let dst = temp.join("dst");
    let result = move_all(&src, &dst);
    println!("  move_all 结果: {result:?}");

    section("? 在 Option 函数中也工作");
    let len = first_word_len("hello world");
    println!("  first_word_len(\"hello world\") = {len:?}");
    let len = first_word_len("");
    println!("  first_word_len(\"\") = {len:?} (None 提前返回)");

    section("Result 与 Option 之间转换");
    let r: Result<i32, &str> = Ok(42);
    let opt: Option<i32> = r.ok();
    println!("  Ok(42).ok() = {opt:?}");

    let r: Result<i32, &str> = Err("失败");
    let opt: Option<i32> = r.ok();
    println!("  Err(_).ok() = {opt:?}");

    // Option → Result：ok_or 提供默认错误
    let opt: Option<i32> = None;
    let r: Result<i32, &str> = opt.ok_or("是 None");
    println!("  None.ok_or(...) = {r:?}");

    section("? 在 main 中（main 可返回 Result）");
    match try_main_logic() {
        Ok(()) => println!("  全部成功"),
        Err(e) => println!("  错误: {e}"),
    }
}

/// 演示 ? 链式使用：每一步都可能失败。
fn try_main_logic() -> Result<(), io::Error> {
    let temp = std::env::temp_dir().join("ch07_demo.txt");
    fs::write(&temp, "hello\n")?; // 写文件
    let content = fs::read_to_string(&temp)?; // 读文件
    println!("  读到: {:?}", content.trim());
    fs::remove_file(&temp)?; // 删文件
    Ok(())
}
