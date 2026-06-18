//! 7.4 处理多种错误类型
//!
//! 关键结论：
//! - 一个函数里可能抛出多种错误（io::Error / ParseIntError / ...）。
//! - 三种统一方案：
//!   1. `Box<dyn Error + Send + Sync + 'static>`（GenericError）：任意错误都能装进来。
//!   2. 自定义错误枚举 + `#[from]` 自动转换（thiserror）。
//!   3. `anyhow::Error`（库）：比 Box<dyn Error> 更顺手。
//! - `?` 通过 `From` trait 做隐式转换。
//! - `err.downcast_ref::<T>()`：把 GenericError 反向转回具体类型。
//!
//! 运行：`cargo run -p ch07_errors --example 04_multiple_errors`

use ch07_errors::{AppError, GenericResult, section};
use std::io::{self, BufRead};

/// 方案 1：用 GenericError 收纳所有错误类型
fn read_numbers_generic(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?; // io::Error → 自动转 Box<dyn Error>
        numbers.push(line.trim().parse()?); // ParseIntError → 自动转
    }
    Ok(numbers)
}

/// 方案 2：用 thiserror 定义统一错误枚举（在 lib.rs 的 AppError）
fn read_numbers_app(file: &mut dyn BufRead) -> Result<Vec<i64>, AppError> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.trim().parse()?);
    }
    Ok(numbers)
}

fn main() {
    section("GenericError：用 ? 把多种错误收进 Box<dyn Error>");

    // 用内存 Cursor 模拟一个文件
    let content = "10\n20\nnot_a_number\n40\n";
    let mut cursor = io::Cursor::new(content);
    match read_numbers_generic(&mut cursor) {
        Ok(nums) => println!("  成功: {nums:?}"),
        Err(err) => {
            println!("  失败: {err}");
            println!("  错误类型: {}", std::any::type_name_of_val(&*err));
        }
    }

    section("GenericError：用 downcast_ref 反向取回具体错误类型");

    let mut cursor = io::Cursor::new(content);
    if let Err(err) = read_numbers_generic(&mut cursor) {
        // 尝试转回 ParseIntError
        if let Some(parse_err) = err.downcast_ref::<std::num::ParseIntError>() {
            println!("  底层是 ParseIntError: {parse_err}");
        } else {
            println!("  底层不是 ParseIntError");
        }
    }

    section("自定义枚举错误（thiserror 自动 #[from]）");

    let mut cursor = io::Cursor::new(content);
    match read_numbers_app(&mut cursor) {
        Ok(nums) => println!("  成功: {nums:?}"),
        Err(AppError::ParseInt(e)) => println!("  AppError::ParseInt: {e}"),
        Err(AppError::Io(e)) => println!("  AppError::Io: {e}"),
        Err(e) => println!("  其他: {e}"),
    }

    section("GenericError::from() 手动转换");

    let io_err = std::io::Error::other("timed out");
    let generic: ch07_errors::GenericError = Box::new(io_err);
    println!("  手动构造 GenericError: {generic}");

    section("三种方案对比");
    println!("  1. GenericError(Box<dyn Error>):");
    println!("     + 简单、任意错误可塞入；- 调用方不知具体错误类型");
    println!("  2. 自定义枚举 + thiserror:");
    println!("     + 类型安全、文档清晰；- 每加一种错误就要改枚举");
    println!("  3. anyhow::Error:");
    println!("     + 像 GenericError 但更顺手、有 context() 链");
}
