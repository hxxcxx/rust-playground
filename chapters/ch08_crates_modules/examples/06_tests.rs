//! 8.6 测试：单元测试 / 集成测试 / 文档测试
//!
//! 关键结论：
//! - 单元测试：`#[test]` 函数 + `#[cfg(test)] mod tests { ... }` 放在源码里。
//! - 集成测试：`tests/` 目录下独立的 .rs 文件，从外部把 crate 当依赖来测。
//! - 文档测试：`///` 注释里的 ```rust 代码块，rustdoc 自动编译运行。
//! - `assert!` / `assert_eq!` / `assert_ne!`：标准断言宏。
//! - `#[should_panic(expected = "...")]`：期望函数 panic。
//! - 测试函数可返回 `Result<(), E>`，用 `?` 简化。
//! - `cargo test` 一次跑全部；`cargo test name` 过滤；`--test-threads 1` 串行。
//!
//! 运行：`cargo run -p ch08_crates_modules --example 06_tests`

use ch08_crates_modules::section;

fn main() {
    section("assert! / assert_eq! / assert_ne!");
    // 用变量避免 clippy 把常量比较当 dead code
    let a = 1 + 1;
    let b = 2 + 2;
    assert!(a == 2);
    assert_eq!(b, 4);
    assert_ne!(a, b);
    println!("  三个断言宏全部通过 (a={a}, b={b})");

    section("自定义失败消息");
    let x = 10;
    assert!(x > 5, "x 应该大于 5，实际是 {x}");
    println!("  x = {x}，断言通过");

    section("浮点数比较：不要用 ==");
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }
    let pi = std::f64::consts::PI;
    assert!(roughly_equal(pi.sin(), 0.0));
    println!("  sin(π) ≈ 0 ✓（用 roughly_equal）");

    section("Result 返回的测试：用 ? 代替 match");
    fn parse_and_check() -> Result<(), std::num::ParseIntError> {
        let n: i32 = "1024".parse()?;
        assert_eq!(n, 1024);
        Ok(())
    }
    parse_and_check().expect("应该成功");
    println!("  返回 Result 的「测试」通过");

    section("三种测试形式总结");
    println!("  1. 单元测试  #[test] + #[cfg(test)] mod tests");
    println!("  2. 集成测试  tests/*.rs，外部视角");
    println!("  3. 文档测试  /// 注释中的 ```rust 代码块");
    println!();
    println!("  运行命令：");
    println!("    cargo test                         # 跑全部");
    println!("    cargo test name_filter             # 按名过滤");
    println!("    cargo test -- --test-threads 1     # 单线程");
    println!("    cargo test -- --nocapture          # 显示 println! 输出");
}

// === 演示一个完整的「单元测试模块」 ===
// 它们不会在 example 的 main 中运行，但会被 `cargo test` 发现。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roughly_equal_basic() {
        assert!((1.0_f64 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_assertions() {
        assert_eq!(2 + 2, 4);
        assert_ne!(1, 2);
    }

    /// `#[should_panic]`：期望函数 panic
    #[test]
    #[should_panic(expected = "divide by zero")]
    #[allow(unconditional_panic, clippy::zero_divided_by_zero)]
    fn test_divide_by_zero_panics() {
        let _ = 1 / 0;
    }

    /// 测试函数返回 Result：可以用 ?
    #[test]
    fn test_parse_returns_result() -> Result<(), std::num::ParseIntError> {
        let n: i32 = "42".parse()?;
        assert_eq!(n, 42);
        Ok(())
    }
}
