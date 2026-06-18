//! 第6章 表达式 —— 共享工具与示例类型。
//!
//! 本章核心：
//! - Rust 是「表达式语言」：`if`/`match`/`{}`/`loop` 都是表达式，会产出值。
//! - 分号 `;` 把「表达式」变成「语句」，丢弃其值；块最后一条不带分号的表达式即块值。
//! - 控制流（`if`/`match`/`if let`/`while let`/`for`/`loop`）都围绕表达式展开。
//! - `break`/`continue` 可带标签；`loop` 可携带值；发散类型 `!`。
//! - 涡轮鱼 `::<>`、自动解引用、范围（半开 `..` 与闭区间 `..=`）、`as` 类型转换。

/// 打印带标题的分割线，便于在 example 输出中区分小节。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

/// 演示「方法调用链」的目标类型：二维点。
/// 同时用来讲解 `.field` 字段访问与 `.` 自动解引用。
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// 类型关联函数（构造器）。
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// 不可变方法（`&self`）：返回新点，演示「表达式返回值」。
    pub fn translated(self, dx: f64, dy: f64) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    /// 可变方法（`&mut self`）：就地平移，返回 `()`。
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}
