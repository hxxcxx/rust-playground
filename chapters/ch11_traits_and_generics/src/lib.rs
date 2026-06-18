//! 第11章 特性与泛型（Traits and Generics）—— 共享工具与示例类型。
//!
//! 本章核心：
//! - trait（特性）：Rust 中描述「类型能做什么」的抽象，类似 Java/C# 接口、
//!   Haskell type class、Swift protocol；但允许为「他人类型」追加方法（孤儿规则下）。
//! - 泛型（generics）：`fn min<T: Ord>(...)`、`struct Vec<T>`、`impl<T> ...`，
//!   单态化（monomorphization）—— 编译期为每个具体类型生成一份代码，零开销。
//! - trait bound（约束）：用 `:` 限制泛型必须实现的 trait；`where` 子句让签名更清爽。
//! - impl Trait：函数参数（限制入参）/ 返回值（隐藏具体类型）的「简写」。
//! - trait object（`dyn Trait`）：运行时多态（动态分发，有 vtable 开销）。
//! - 关联项（associated items）：关联类型 / 关联常量 / 关联函数（`Iterator` 就是典型）。
//!
//! 静态 vs 动态：
//! - 泛型 + trait bound → 静态分发（编译期决定，性能最好，但代码体积增大）。
//! - Box<dyn Trait>     → 动态分发（运行期查 vtable，同容器可装不同类型）。

// =======================================================================
// 工具函数
// =======================================================================

/// 打印带标题的分割线（与前几章风格一致）。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

// =======================================================================
// 例 1：一个最朴素的 trait —— 用于演示「定义 / 实现 / 调用」
// =======================================================================

/// 拥有「面积」的东西。
///
/// 这是书中讲解 trait 时反复用到的例子，对应正文：
/// - `trait Trait { fn method(&self) -> T; }` 定义方法签名（无函数体）。
/// - `impl Trait for Type { fn method(&self) -> T { ... } }` 为类型实现 trait。
pub trait IsShape {
    /// 返回面积。
    fn area(&self) -> f64;
    /// 默认方法：所有实现者免费获得，可选择覆盖。
    fn name(&self) -> &str {
        "unknown shape"
    }
}

// =======================================================================
// 例 2：用于演示 trait object 的几何类型
// =======================================================================

/// 一个二维点。
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// 圆。
#[derive(Copy, Clone, Debug)]
pub struct Circle {
    pub radius: f64,
}

impl IsShape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn name(&self) -> &str {
        "circle"
    }
}

/// 矩形。
#[derive(Copy, Clone, Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl IsShape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn name(&self) -> &str {
        "rectangle"
    }
}

// =======================================================================
// 例 3：超特性（supertrait）—— 需要实现者「先」实现别的 trait
// =======================================================================

/// 任何可以「渲染成文本」的东西。
pub trait IsVisible {
    /// 返回用于显示的字符串。
    fn draw(&self) -> String;
}

/// 「可见 + 可点击」的 UI 组件。
///
/// `: IsVisible` 表示 IsClickable 是 IsVisible 的「子特性」，
/// 实现 IsClickable 的类型必须**同时**实现 IsVisible。
pub trait IsClickable: IsVisible {
    /// 被点击时的回调名。
    fn on_click(&self) -> &str;
}

// =======================================================================
// 例 4：为「他人类型」（标准库 Vec）实现自定义 trait —— 演示孤儿规则
// =======================================================================

/// 能写出自身「总结信息」的东西。
pub trait Summary {
    fn summarize(&self) -> String;
}

// 孤儿规则（Orphan Rule）：只能在自己 crate 定义的 trait 上，
// 为「任意」类型实现；或在自己的类型上为「任意」trait 实现。
// 这里 trait Summary 是本 crate 的，所以可以 for 标准库 Vec。
impl<T: Summary> Summary for Vec<T> {
    fn summarize(&self) -> String {
        let parts: Vec<String> = self.iter().map(|item| item.summarize()).collect();
        format!("[{} 项: {}]", self.len(), parts.join(", "))
    }
}

/// 一条简短的新闻条目（自带 Summary）。
#[derive(Clone, Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} ({})", self.headline, self.location)
    }
}

// 为「原始类型 i32」实现本 crate 的 Summary：
// 这必须在「定义 Summary 的 crate」里写 ——
// 因为对原始类型 impl 有额外限制（即便 trait 是本 crate 的，
// 也得在 trait 所在 crate 中实现）。
// 外部 crate 想给 i32 加 trait？只能用 newtype 模式（见 03 例）。
impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("i32:{}", self)
    }
}

// =======================================================================
// 例 5：关联类型（associated type）—— 像 Iterator 一样
// =======================================================================

/// 一个线性序列，每次能取出「下一个」元素。
///
/// 关联类型 `Item` 让实现者**指定**自己产出什么类型，
/// 调用方不必再写 `MyIter<i32>` 这种参数化。
pub trait MyIterator {
    /// 每次迭代产出的元素类型（由实现者决定）。
    type Item;

    /// 取下一个元素；None 表示结束。
    fn next(&mut self) -> Option<Self::Item>;
}

/// 一个简单的计数器：从 start 数到 end（不含）。
pub struct Counter {
    pub current: i32,
    pub end: i32,
}

impl Counter {
    pub fn new(start: i32, end: i32) -> Self {
        Self { current: start, end }
    }
}

impl MyIterator for Counter {
    // 实现者在这里固定关联类型：
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let v = self.current;
            self.current += 1;
            Some(v)
        } else {
            None
        }
    }
}

// =======================================================================
// 例 6：关联常量 —— 由实现者提供的「编译期常量」
// =======================================================================

/// 具有几何「零维度」定义的类型，由关联常量给出。
pub trait HasZeroDimension {
    /// 该类型对应的「零」维度值（编译期常量）。
    const ZERO: f64;
    /// 单位标签（字符串常量）。
    const UNIT: &'static str;
}

impl HasZeroDimension for Circle {
    const ZERO: f64 = 0.0;
    const UNIT: &'static str = "radius";
}

impl HasZeroDimension for Rectangle {
    const ZERO: f64 = 0.0;
    const UNIT: &'static str = "width/height";
}
