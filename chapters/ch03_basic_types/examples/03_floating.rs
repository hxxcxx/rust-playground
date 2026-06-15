//! 3.3 浮点类型：IEEE 754 单精度/双精度、特殊值、数学方法
//!
//! 运行：`cargo run -p ch03_basic_types --example 03_floating`
//
// 教学性示例：故意演示 NaN == NaN 的 IEEE 特性（恒为 false）。
#![allow(invalid_nan_comparisons)]

use std::f64::consts::{E, PI};

use ch03_basic_types::section;

fn main() {
    section("字面量：整数部分后可省略，但小数/指数/类型后缀至少一个");
    // 数组必须同类型 —— 这里统一为 f64
    let floats: [f64; 6] = [-1.5625, 2., 0.25, 1e4, 40f64, 9.109_383_56e-31];
    for v in floats {
        println!("{v}");
    }
    // f32 后缀字面量单独演示
    let _: f32 = 40f32;

    section("默认类型推断：双类型皆可时选 f64");
    let x = 1.0; // 默认 f64
    println!(
        "默认推断为 f64: {}, std::mem::size_of = {} bytes",
        x,
        size_of_val(&x)
    );

    section("IEEE 特殊值");
    assert!((-1.0_f32 / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN, f32::MAX); // MIN 是最小有限值（负数），不是负无穷
    assert!(f32::NAN.is_nan());
    assert!(f32::INFINITY > f32::MAX);
    println!("+∞ > f32::MAX ? {}", f32::INFINITY > f32::MAX);
    println!(
        "NaN 任何比较都为 false：NaN == NaN ? {}",
        f32::NAN == f32::NAN
    );

    section("数学方法");
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // IEEE 精确保证
    assert_eq!((-1.01f64).floor(), -2.0);
    assert_eq!(2.7f64.ceil(), 3.0);
    assert_eq!(2.5f64.round(), 3.0); // 四舍五入
    println!("5f32.sqrt()² = {}", 5f32.sqrt() * 5f32.sqrt());
    println!("(-1.01).floor() = {}", (-1.01f64).floor());

    section("常用常量（std::f64::consts）");
    println!("π = {PI}");
    println!("e = {E}");
    println!("√2 = {}", std::f64::consts::SQRT_2);
    println!("sin(π/2) = {}", (PI / 2.0).sin());

    section("`as` 浮点 ↔ 整数：向 0 截断");
    assert_eq!(3.9_f64 as i32, 3); // 截断小数
    assert_eq!((-3.9_f64) as i32, -3); // 向 0 截断
    assert_eq!(42_i32 as f64, 42.0);
    println!("3.9 as i32 = {}", 3.9_f64 as i32);

    section("注意：Rust 没有隐式数值转换");
    // let bad: f64 = 5_i32; // 编译错误！
    let ok: f64 = 5_i32 as f64; // 必须显式
    println!("显式转换 5_i32 as f64 = {ok}");
}
