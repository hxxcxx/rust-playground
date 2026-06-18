//! 第7章 错误处理 —— 共享工具与示例类型。
//!
//! 本章核心：
//! - Rust 用两种机制处理错误：
//!   * `panic!` —— 程序员失误（不该发生），默认展开栈，可改成终止进程。
//!   * `Result<T, E>` —— 普通错误（外部因素导致：IO、网络、用户输入）。
//! - `?` 运算符：一行代码完成「错误传播」，并自动做 `From` 转换。
//! - 多种错误类型 → 定义统一错误枚举，或用 `Box<dyn Error>` / `anyhow::Error`。
//! - 自定义错误类型要实现 `Display` + `std::error::Error`（或用 `thiserror`）。

use std::fmt;

/// 打印带标题的分割线。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

/// 演示「自定义错误类型」的最小骨架：手动实现 `Display` + `Error`。
#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl JsonError {
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        JsonError {
            message: message.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

// Error trait 的默认实现已经够用
impl std::error::Error for JsonError {}

/// 用 `thiserror` 自动派生错误枚举：跨函数传播多个底层错误时的标准做法。
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("I/O 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("解析整数失败: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("JSON 错误: {0}")]
    Json(#[from] JsonError),
}

/// 「通用错误」类型别名：可以用 `?` 把任意 `Error + Send + Sync + 'static` 转进来。
pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type GenericResult<T> = Result<T, GenericError>;

/// 递归打印错误链：错误 → caused by → caused by …
pub fn print_error(mut err: &dyn std::error::Error) {
    eprintln!("error: {err}");
    while let Some(source) = err.source() {
        eprintln!("caused by: {source}");
        err = source;
    }
}
