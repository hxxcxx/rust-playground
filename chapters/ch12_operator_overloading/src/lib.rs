//! 第12章 运算符重载（Operator Overloading）—— 共享类型与示例。
//!
//! 本章核心：
//! - Rust 没有「自定义运算符」，但可以通过实现 `std::ops` / `std::cmp` 里的 trait，
//!   让 `+ - * / % ! & | << >>` 以及 `== < > [i]` 等运算符对自己的类型生效。
//! - 每个「运算符」对应一个 trait，里面通常只有一个方法（如 `Add::add`）。
//! - 算术 trait（`Add/Sub/Mul/Div/Rem`）带关联类型 `Output`，决定运算结果类型。
//! - 复合赋值（`+= -= ...`）有独立的 trait（`AddAssign` 等），按需实现。
//! - `Rhs` 泛型参数：默认是 `Self`，但可以改成允许「不同类型」相加（如 `Vec * scalar`）。
//! - 比较：`PartialEq`（== / !=）/ `PartialOrd`（< > <= >=）；派生最常用。
//! - 索引：`Index`（只读 `v[i]`）/ `IndexMut`（可写 `v[i] = x`）。
//!
//! 关键理念：运算符重载只是「trait + 方法的语法糖」——
//! `a + b` 完全等价于 `Add::add(a, b)`，没有魔法。

use std::ops::{Add, AddAssign, Neg};

// =======================================================================
// 工具函数
// =======================================================================

/// 打印带标题的分割线（与前几章风格一致）。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

// =======================================================================
// 复数：本章贯穿示例（来自《Programming Rust》）
// =======================================================================

/// 复数：演示「算术运算符 + 一元负 + 复合赋值 + 比较」全流程。
///
/// 泛型 T 表示实部/虚部的底层数值类型（i32 / f64 ...），
/// 通过 trait bound 约束「可以做算术」的类型。
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    /// 构造一个复数。
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }
}

// ---- 复数加法：impl Add for Complex<T> ----
// `a + b` 会调用这里的 add()。
// 注意 Output 也是 Complex<T> ——「复数加复数得复数」。
impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

// ---- 复数复合赋值：impl AddAssign ----
// 让 `c1 += c2;` 能工作（+= 不是自动从 + 派生的，要单独实现）。
impl<T> AddAssign for Complex<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

// ---- 一元负：impl Neg（让 `-c` 工作）----
impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

// =======================================================================
// 泛型运算符：Vec * 标量（演示 Rhs 泛型参数）
// =======================================================================

/// 让 `Vec<T> * scalar` 工作：用「自定义 Rhs」实现 `Mul`。
///
/// 标准库的 `Mul` 默认 `Rhs = Self`，这里改成 `Rhs = T`（标量），
/// 这样 `vector * 2.0` 才合法。
pub trait MulScalar<Rhs = Self> {
    /// 乘法结果类型。
    type Output;
    /// `self * rhs`。
    fn mul_scalar(self, rhs: Rhs) -> Self::Output;
}

// 为 Vec<f64> 实现「乘以标量」。
impl MulScalar<f64> for Vec<f64> {
    type Output = Vec<f64>;
    fn mul_scalar(self, scalar: f64) -> Vec<f64> {
        self.into_iter().map(|x| x * scalar).collect()
    }
}

// =======================================================================
// 位运算示例类型
// =======================================================================

/// 一个「位掩码」集合：演示 BitAnd / BitOr / BitXor / Not。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Bitmask(pub u32);

impl std::ops::BitAnd for Bitmask {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Bitmask(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for Bitmask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Bitmask(self.0 | rhs.0)
    }
}

impl std::ops::BitXor for Bitmask {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Bitmask(self.0 ^ rhs.0)
    }
}

impl std::ops::Not for Bitmask {
    type Output = Self;
    fn not(self) -> Self {
        Bitmask(!self.0)
    }
}

// =======================================================================
// 索引示例类型：一个固定大小的二维网格
// =======================================================================

/// 4×4 的字节网格：演示 Index / IndexMut（让 `grid[(row, col)]` 工作）。
#[derive(Clone, Debug)]
pub struct Grid {
    pub cells: [u8; 16],
}

impl Grid {
    pub fn new() -> Self {
        Self { cells: [0; 16] }
    }

    /// 把 (row, col) 折叠成一维下标。
    fn index_of((row, col): (usize, usize)) -> usize {
        row * 4 + col
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

// Index：只读索引 `grid[(r, c)]` 返回 u8 的拷贝。
impl std::ops::Index<(usize, usize)> for Grid {
    type Output = u8;
    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        &self.cells[Self::index_of(pos)]
    }
}

// IndexMut：可写索引 `grid[(r, c)] = v`。
// 实现了 IndexMut 自动获得 &mut self 的 [i] 写入能力。
impl std::ops::IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        &mut self.cells[Self::index_of(pos)]
    }
}
