//! 第4章 所有权与移动 —— 共享工具与示例类型。
//!
//! 本章核心：
//! - 每个值有唯一所有者；所有者离开作用域时值被「丢弃」（drop）。
//! - 大多数类型赋值/传参是「移动」（move），源变量变为未初始化。
//! - `Copy` 类型（整数、浮点、`char`、`bool` 等）赋值是按位复制，源变量仍可用。
//! - `Rc` / `Arc` 提供引用计数式「共享所有权」。

/// 打印带标题的分割线，便于在 example 输出中区分小节。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

/// 本章反复使用的结构体示例：`name` 拥有堆上的 `String`，`birth` 是 `i32`。
/// `Person` 拥有 `name`，因此 `Person` 被 drop 时，`String` 的缓冲区也会被释放。
pub struct Person {
    pub name: String,
    pub birth: i32,
}

/// 演示「`Copy` 类型」的用户自定义类型：仅含一个 `u32` 字段。
/// 由于字段是 `Copy`，整个结构体也可以声明为 `Copy`。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Label {
    pub number: u32,
}
