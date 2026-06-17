//! 第5章 引用 —— 共享工具与示例类型。
//!
//! 本章核心：
//! - 引用是「非拥有所有权的指针」，分共享引用 `&T`（Copy，可多个）和可变引用 `&mut T`（非 Copy，独占）。
//! - 引用的生命周期不能超过它指向的值 —— 必须在代码中体现，编译期强制检查。
//! - 共享与可变互斥：共享引用期间值被「冻结」为只读；可变引用期间值「独占」不可它路访问。
//! - 这两条规则是 Rust 内存安全 + 无数据竞争的编译期基石。

use std::collections::HashMap;

/// 打印带标题的分割线，便于在 example 输出中区分小节。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

/// 本章反复使用的「艺术家 → 作品列表」哈希表类型。
/// 因为 `String` / `Vec` 都不是 `Copy`，`Table` 也不是 ——
/// 按值传递会移动整个结构。这正是需要「引用」的典型场景。
pub type Table = HashMap<String, Vec<String>>;

/// 构造一个示例 Table，供多个 example 共享。
pub fn sample_table() -> Table {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );
    table
}

/// 简单二维点，用于演示「引用的引用」、「字段借用」等。
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// 用于演示「带生命周期参数的结构体」。
/// `r` 字段持有 `&'a i32`，因此整个 `S` 必须带上生命周期参数 `'a`。
/// `'a` 表示：`S` 中引用的对象生命周期必须包围 `S` 自身的生命周期。
pub struct S<'a> {
    pub r: &'a i32,
}

/// 演示「两个独立生命周期参数」：`x` 和 `y` 各自有独立的生命周期，
/// 不会因为其中之一较短的引用而过度限制另一个。
pub struct TwoRefs<'a, 'b> {
    pub x: &'a i32,
    pub y: &'b i32,
}
