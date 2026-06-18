//! 12.1 算术运算符：Add / Sub / Mul / Div / Rem
//!
//! 关键结论：
//! - `a + b` 等价于 `Add::add(a, b)` —— 运算符就是 trait 方法的语法糖。
//! - 每个算术运算符对应一个 trait（都在 `std::ops`）：
//!   `+` Add / `-` Sub / `*` Mul / `/` Div / `%` Rem。
//! - 这些 trait 都带关联类型 `Output`，决定结果类型：
//!   `impl Add for T { type Output = ...; fn add(...) -> ... }`。
//! - 默认 `Rhs = Self`（左右操作数同类型），可以改成别的（见 03_generic_rhs）。
//! - 派生 Debug/Clone/Copy 后能直接配合运算符使用，无需手写。
//!
//! 运行：`cargo run -p ch12_operator_overloading --example 01_arithmetic`

use ch12_operator_overloading::{Complex, section};
use std::ops::{Add, Div, Mul, Sub};

fn main() {
    section("复数加法：a + b 调用 Complex::add");
    let a = Complex::new(1.0, 2.0); // 1 + 2i
    let b = Complex::new(3.0, 4.0); // 3 + 4i
    // 这一行等价于：<Complex as Add>::add(a, b)
    let c = a + b;
    println!("  ({a:?}) + ({b:?}) = {c:?}"); // 4 + 6i

    section("内置类型本身也实现了这些 trait");
    // i32 实现了 Add<Output = i32>，所以 + 能用。
    let n: i32 = 5 + 3;
    let m: i32 = Add::add(5, 3); // 完全等价的写法
    println!("  5 + 3 = {n}");
    println!("  Add::add(5, 3) = {m}");

    section("演示 Output 可以与输入不同");
    // 这里写一个：把两个 i32 「加」起来得到 f64（人为规定 Output = f64）。
    #[derive(Copy, Clone, Debug)]
    struct Promoting(i32);
    impl Add for Promoting {
        type Output = f64; // 注意：Output 不是 Self
        fn add(self, rhs: Self) -> f64 {
            (self.0 + rhs.0) as f64
        }
    }
    let p = Promoting(2) + Promoting(3);
    println!("  Promoting(2) + Promoting(3) = {p} (类型 f64)");

    section("四则运算全实现：自定义 2D 向量");
    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = Vec2 { x: 3.0, y: 4.0 };
    println!("  v1 + v2 = {:?}", v1 + v2);
    println!("  v1 - v2 = {:?}", v1 - v2);
    println!("  v1 * 2.0 = {:?}", v1 * Vec2 { x: 2.0, y: 2.0 }); // 逐分量乘
    println!("  v1 / v2 = {:?}", v1 / v2);
}

/// 二维向量：演示完整的四则运算符实现。
#[derive(Copy, Clone, Debug, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div for Vec2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
