//! 第14章 闭包（Closures）—— 共享工具与示例类型。
//!
//! 本章核心：
//! - 闭包：`|参数| 表达式`，是「可以捕获环境变量」的匿名函数（类似 lambda）。
//! - 捕获环境变量的三种方式（编译器自动选择，按「最小权限」）：
//!   * 借用：`&T`（最常见，只读）
//!   * 可变借用：`&mut T`
//!   * 移动：`T`（用 `move` 关键字强制）
//! - 闭包作为参数：三个标准 trait：
//!   * `Fn`      —— 调用时不改变环境（可多次调用）
//!   * `FnMut`   —— 调用时可变借用环境（可多次调用）
//!   * `FnOnce`  —— 调用时获取环境所有权（只能调用一次）
//!
//!   它们是「子 trait」关系：Fn ⊂ FnMut ⊂ FnOnce。
//! - 闭包作为返回值：用 `impl Fn(...)`（或 Box<dyn Fn>）。
//! - 性能：闭包编译为「匿名结构体 + 方法」，调用是直接调用 —— 零开销。

// =======================================================================
// 工具函数
// =======================================================================

/// 打印带标题的分割线（与前几章风格一致）。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

// =======================================================================
// 示例类型：SortWriter / Account（演示闭包捕获场景）
// =======================================================================

/// 账户：演示「闭包捕获结构体字段」。
#[derive(Debug, Clone)]
pub struct Account {
    pub owner: String,
    pub balance: i64,
}

impl Account {
    /// 返回一个「判断余额是否超过阈值」的闭包。
    /// 闭包捕获了 self（其实是 self.threshold 的副本）。
    pub fn is_rich_checker(&self, threshold: i64) -> impl Fn(&Account) -> bool + '_ {
        // 这里 threshold 是 i64（Copy），闭包按值捕获它。
        move |acc: &Account| acc.balance >= threshold
    }
}

/// 一个简单的可排序元素：演示「自定义排序键」闭包。
#[derive(Debug, Clone)]
pub struct City {
    pub name: String,
    pub population: u64,
}

impl City {
    pub fn new(name: &str, population: u64) -> Self {
        Self {
            name: name.to_string(),
            population,
        }
    }
}

// =======================================================================
// 示例 trait：把「闭包存进结构体」
// =======================================================================

/// 一个带「回调」的结构体：演示把闭包作为字段保存。
///
/// 泛型 F 是闭包类型（实现了 Fn）。
/// 注意：F 必须是泛型参数（每个闭包有独特的匿名类型）。
pub struct Button<F: Fn()> {
    pub label: String,
    pub on_click: F,
}

impl<F: Fn()> Button<F> {
    pub fn new(label: &str, on_click: F) -> Self {
        Self {
            label: label.to_string(),
            on_click,
        }
    }

    /// 模拟「点击」：触发回调。
    pub fn click(&self) {
        (self.on_click)();
    }
}
