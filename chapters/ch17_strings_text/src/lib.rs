//! 第17章 字符串和文本（Strings and Text）—— 共享工具与示例类型。
//!
//! 本章核心：
//! - `String` vs `&str`：
//!   * `String`：堆上、可变、拥有所有权的 UTF-8 字符串（增长型）。
//!   * `&str`：字符串切片，指向某处 UTF-8 字节的「胖指针」（指针 + 长度）。
//! - Rust 字符串「强制 UTF-8」：所有 String/&str 都是合法 UTF-8。
//! - 字节 vs 字符 vs 字形簇：
//!   * bytes() —— 字节迭代（UTF-8 编码单元）
//!   * chars() —— Unicode 标量值迭代（char）
//!   * graphemes —— 字形簇（需要外部 crate，如 unicode-segmentation）
//! - 三种「不是字符串」的字节类型：`Vec<u8>` / `&[u8]` / `b"..."` 字节串。
//! - 格式化：`format!` / `println!` / `format_args!` / Display + Debug trait。
//! - 构建字符串：`push_str` / `extend` / `+` / `write!`。
//! - 解析与格式化：`str::parse` / `ToString` / `format!`。

// =======================================================================
// 工具函数
// =======================================================================

/// 打印带标题的分割线（与前几章风格一致）。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

// =======================================================================
// 示例类型：演示自定义 Display
// =======================================================================

/// 复数：演示手写 Display（控制输出格式）。
#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 用 + 号在正虚部前显示「+」，负的自动带「-」。
        // {:+.0} 表示「带符号、0 位小数」。
        write!(f, "{:.1} {:+.1}i", self.re, self.im)
    }
}

// =======================================================================
// 示例类型：演示「带格式的结构体」（继承结构体对齐宽度）
// =======================================================================

/// 一个带「填充/对齐」输出的矩阵元素。
#[derive(Debug)]
pub struct Matrix {
    pub rows: Vec<Vec<i32>>,
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 找出最大列宽，对齐输出。
        let max = self
            .rows
            .iter()
            .flat_map(|r| r.iter())
            .map(|n| n.to_string().len())
            .max()
            .unwrap_or(1);
        for row in &self.rows {
            for (i, n) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                // 右对齐到 max 宽度。
                write!(f, "{:>width$}", n, width = max)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
