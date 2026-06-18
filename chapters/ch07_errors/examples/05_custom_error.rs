//! 7.5 自定义错误类型
//!
//! 关键结论：
//! - 任何错误类型都应该：
//!   * 实现 `Debug`（用 `{:?}` 打印技术细节）；
//!   * 实现 `Display`（用 `{}` 打印给用户的简短消息）；
//!   * 实现 `std::error::Error`（通常用默认实现就够）。
//! - 手写三个 impl 略繁琐，可用 `thiserror::Error` 派生宏一行搞定。
//! - 错误链：`err.source()` 返回上一级错误，便于追溯根本原因。
//!
//! 运行：`cargo run -p ch07_errors --example 05_custom_error`

use ch07_errors::{AppError, JsonError, print_error, section};
use std::fmt;

/// 手写一个最小错误类型：带文件名和行号
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
}

impl ParseError {
    pub fn new(msg: impl Into<String>, line: usize) -> Self {
        Self {
            message: msg.into(),
            line,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (line {})", self.message, self.line)
    }
}

impl std::error::Error for ParseError {}

fn main() {
    section("手写错误类型：Debug + Display + Error");

    let err = ParseError::new("expected ';' but got '}'", 42);
    println!("  Debug    : {:?}", err);
    println!("  Display  : {err}");
    println!("  source() : {:?}", std::error::Error::source(&err));

    section("lib.rs 中的 JsonError（手写）");
    let err = JsonError::new("expected ']' at end of array", 10, 5);
    println!("  JsonError: {err}");

    section("thiserror 派生：一行就生成 Display + Error + From");
    // AppError 是 #[derive(thiserror::Error)] 生成的，看 lib.rs
    let json_err = JsonError::new("unexpected token", 1, 2);
    let app_err: AppError = json_err.into(); // #[from] 自动实现
    println!("  AppError: {app_err}");
    println!("  source  : {:?}", std::error::Error::source(&app_err));

    section("错误链：递归打印 source");
    // 构造一个嵌套错误链：AppError::Io → 由 io::Error 包装
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "文件不存在");
    let app_err: AppError = io_err.into();
    print_error(&app_err);

    section("抛出自定义错误（模拟函数返回）");
    match parse_config("bad input") {
        Ok(config) => println!("  解析成功: {config}"),
        Err(e) => println!("  解析失败: {e}"),
    }
}

/// 模拟一个会失败的解析函数。
fn parse_config(input: &str) -> Result<String, ParseError> {
    if input.contains("bad") {
        return Err(ParseError::new("invalid keyword", 1));
    }
    Ok(format!("parsed: {input}"))
}
