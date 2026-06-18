//! 12.3 Rhs 泛型参数：让「不同类型」相加 / 相乘
//!
//! 关键结论：
//! - 算术 trait 都带一个泛型参数 Rhs（right-hand side，右操作数类型）：
//!   `trait Add<Rhs = Self> { type Output; fn add(self, rhs: Rhs) -> Output; }`
//! - 默认 `Rhs = Self`（左右同类型），但你可以指定别的 Rhs：
//!   `impl Mul<f64> for Vec<f64>` 让 `vector * 2.0` 合法。
//! - 这让「标量乘向量」「不同单位相加」等需求成为可能。
//! - 可以对「同一类型」多次 impl，只要 Rhs 不同：
//!   `impl Mul<f32> for V` 和 `impl Mul<f64> for V` 可以共存。
//!
//! 运行：`cargo run -p ch12_operator_overloading --example 03_generic_rhs`

use ch12_operator_overloading::{MulScalar, section};
use std::ops::{Add, Mul};

/// 一个带「单位」的量：演示 Rhs = 不同类型时的相加。
#[derive(Debug, Clone, Copy)]
struct Meters(f64);

#[derive(Debug, Clone, Copy)]
struct Millimeters(f64);

// Meters + Millimeters → Meters（左 Meters，右 Millimeters，结果 Meters）
// 这里 Rhs = Millimeters，Output = Meters。
impl Add<Millimeters> for Meters {
    type Output = Meters;
    fn add(self, rhs: Millimeters) -> Meters {
        Meters(self.0 + rhs.0 / 1000.0)
    }
}

fn main() {
    section("Rhs = 不同类型：Meters + Millimeters");
    let length = Meters(1.5);
    let extra = Millimeters(250.0);
    // 如果不指定 Rhs=Millimeters，这里会编译失败（类型不同）。
    let total = length + extra;
    println!("  1.5m + 250mm = {:.3}m", total.0);

    section("标量乘向量：Vec<T> * scalar");
    let v: Vec<f64> = vec![1.0, 2.0, 3.0];
    let scaled = v.mul_scalar(2.5);
    println!("  [1,2,3] * 2.5 = {:?}", scaled);

    section("同一类型对多个 Rhs 实现 Mul（共存）");
    let p = Pixel { r: 100, g: 50, b: 25 };
    // Pixel * f32
    let p1 = p * 0.5_f32;
    println!("  Pixel * 0.5f32 = {:?}", p1);
    // Pixel * f64
    let p2 = p * 0.5_f64;
    println!("  Pixel * 0.5f64 = {:?}", p2);
    // Pixel * i32
    let p3 = p * 2_i32;
    println!("  Pixel * 2i32   = {:?}", p3);

    section("为外来类型（标准库 Vec）实现本地 trait（MulScalar）");
    // 这是第 11 章孤儿规则的应用：trait 是本 crate 的，可以给标准库 Vec 实现。
    let v2: Vec<f64> = vec![10.0, 20.0];
    let doubled = v2.mul_scalar(3.0);
    println!("  [10,20] * 3.0 = {:?}", doubled);
}

/// 像素：演示「对多个 Rhs 各实现一次 Mul」。
#[derive(Debug, Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Mul<f32> for Pixel {
    type Output = Pixel;
    fn mul(self, s: f32) -> Pixel {
        Pixel {
            r: (self.r as f32 * s).clamp(0.0, 255.0) as u8,
            g: (self.g as f32 * s).clamp(0.0, 255.0) as u8,
            b: (self.b as f32 * s).clamp(0.0, 255.0) as u8,
        }
    }
}

impl Mul<f64> for Pixel {
    type Output = Pixel;
    fn mul(self, s: f64) -> Pixel {
        Pixel {
            r: (self.r as f64 * s).clamp(0.0, 255.0) as u8,
            g: (self.g as f64 * s).clamp(0.0, 255.0) as u8,
            b: (self.b as f64 * s).clamp(0.0, 255.0) as u8,
        }
    }
}

impl Mul<i32> for Pixel {
    type Output = Pixel;
    fn mul(self, s: i32) -> Pixel {
        Pixel {
            r: (self.r as i32 * s).clamp(0, 255) as u8,
            g: (self.g as i32 * s).clamp(0, 255) as u8,
            b: (self.b as i32 * s).clamp(0, 255) as u8,
        }
    }
}
