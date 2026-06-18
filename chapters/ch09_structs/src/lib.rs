//! 第9章 结构体 —— 共享工具与示例类型。
//!
//! 本章核心：
//! - 三种结构体：具名字段 / 元组结构体 / 类单元结构体。
//! - `impl` 块定义方法与关联函数；`self`/`&self`/`&mut self`/`Box<Self>` 等。
//! - 关联常量（`impl` 块里的 `const`）；类型关联函数（无 `self` 参数的 `fn`）。
//! - 泛型结构体 `struct Queue<T>`；带生命周期参数 `struct Extrema<'a>`。
//! - `#[derive]` 派生常用 trait：Debug/Clone/Copy/PartialEq/Eq/Hash 等。
//! - 内部可变性：`Cell<T>`（Copy 类型）/ `RefCell<T>`（运行时借用检查）。

use std::cell::{Cell, RefCell};

/// 打印带标题的分割线。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

// =======================================================================
// 具名字段结构体
// =======================================================================

/// 八位灰度位图：演示具名字段结构体 + 公共/私有字段。
pub struct GrayscaleMap {
    pub pixels: Vec<u8>,
    pub size: (usize, usize),
}

impl GrayscaleMap {
    /// 类型关联函数（构造器）：名字不一定是 new，但约定俗成。
    pub fn new(size: (usize, usize)) -> Self {
        let (w, h) = size;
        Self {
            pixels: vec![0; w * h],
            size,
        }
    }
}

// =======================================================================
// 元组结构体
// =======================================================================

/// 二维边界：演示元组结构体（字段用 .0 .1 访问）。
pub struct Bounds(pub usize, pub usize);

/// Newtype 模式：包一个 Vec<u8>，获得更严格的类型检查。
pub struct Ascii(pub Vec<u8>);

// =======================================================================
// 类单元结构体
// =======================================================================

/// 没有字段的结构体，类似单元类型 `()`。
/// 在 trait 实现中很有用（第 11 章）。
pub struct Onesuch;

// =======================================================================
// 泛型结构体：先进先出队列
// =======================================================================

/// 泛型队列：可存储任意类型 T。
pub struct Queue<T> {
    older: Vec<T>,   // 旧元素，最早的在末尾
    younger: Vec<T>, // 新元素，最新的在末尾
}

impl<T> Queue<T> {
    /// 构造空队列。
    pub fn new() -> Self {
        Self {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    /// 入队（不可变方法看起来可变：因为 `&mut self`）。
    pub fn push(&mut self, t: T) {
        self.younger.push(t);
    }

    /// 出队：返回 Some(旧值) 或 None。
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            // 把 younger 全部反转搬到 older
            std::mem::swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    /// 是否为空（共享引用方法）。
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    /// 按值 self：消耗队列，分离两个 Vec。
    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 为特定类型 Queue<f64> 实现专属方法：sum。
/// 这种「impl Queue<f64>」只能定义在本 crate 内（孤儿规则）。
impl Queue<f64> {
    /// 把队列中所有 f64 求和（其他类型的 Queue 没有这个方法）。
    pub fn sum(&self) -> f64 {
        self.older.iter().chain(self.younger.iter()).sum()
    }
}

// =======================================================================
// 带生命周期参数的结构体
// =======================================================================

/// 持有切片中最大、最小元素的两个引用。
/// 'elt 表示两个引用的生命周期必须相同（且不能超过被引用的数据）。
pub struct Extrema<'elt> {
    pub greatest: &'elt i32,
    pub least: &'elt i32,
}

/// 在切片中查找极值，返回的结构体引用同一切片。
pub fn find_extrema(slice: &[i32]) -> Extrema<'_> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];
    for v in slice.iter().skip(1) {
        if v < least {
            least = v;
        }
        if v > greatest {
            greatest = v;
        }
    }
    Extrema { greatest, least }
}

// =======================================================================
// 演示内部可变性：Cell + RefCell
// =======================================================================

/// 演示内部可变性：在「不可变」结构体内有「可变」字段。
/// `Cell<u32>` 适合 Copy 类型；`RefCell<File>` 适合非 Copy 类型（运行时借用检查）。
pub struct SpiderRobot {
    pub species: String,
    /// 计数器：用 Cell 实现「不可变 self 也能修改」。
    pub hardware_error_count: Cell<u32>,
}

impl SpiderRobot {
    pub fn new(species: &str) -> Self {
        Self {
            species: species.to_string(),
            hardware_error_count: Cell::new(0),
        }
    }

    /// 注意：`&self`（不可变），但能修改计数器！这就是内部可变性。
    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }

    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }
}

/// 用 RefCell 持有 String：演示运行时借用检查。
pub struct LogBuffer {
    pub lines: RefCell<Vec<String>>,
}

impl Default for LogBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl LogBuffer {
    pub fn new() -> Self {
        Self {
            lines: RefCell::new(Vec::new()),
        }
    }

    /// `&self` 添加日志行 —— 通过 RefCell 借用可变引用。
    pub fn log(&self, line: impl Into<String>) {
        self.lines.borrow_mut().push(line.into());
    }

    /// 返回当前所有日志的拷贝。
    pub fn snapshot(&self) -> Vec<String> {
        self.lines.borrow().clone()
    }
}
