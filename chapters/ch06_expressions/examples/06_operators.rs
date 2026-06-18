//! 6.6 算术 / 按位 / 比较 / 逻辑运算符
//!
//! 关键结论：
//! - Rust 的二元运算符与 C 类似，但有几个差异：
//!   * 整数溢出在 debug 模式 panic；用 `wrapping_*`、`checked_*`、`saturating_*`。
//!   * 按位取反是 `!`（不是 C 的 `~`），逻辑非也是 `!`。
//!   * 没有自增/自减 `++`/`--`，没有复合赋值链 `a = b = 3`。
//!   * 按位运算优先级 > 比较运算（与 C 相反，更符合直觉）。
//!   * `%` 可用于浮点数。
//! - 所有比较运算两侧类型必须相同。
//!
//! 运行：`cargo run -p ch06_expressions --example 06_operators`

use ch06_expressions::section;

fn main() {
    section("算术 + 检查溢出方法");
    let a: i32 = 10;
    let b: i32 = 3;
    println!("  {a} + {b} = {}", a + b);
    println!("  {a} - {b} = {}", a - b);
    println!("  {a} * {b} = {}", a * b);
    println!("  {a} / {b} = {}", a / b); // 整数除法：向零取整 → 3
    println!("  {a} % {b} = {}", a % b); // 余数符号跟左操作数

    // checked_div：除零返回 None 而非 panic
    let safe = 10_i32.checked_div(0);
    println!("  10 / 0 (checked) = {safe:?}");

    // wrapping_add：忽略溢出，环绕
    let max = i32::MAX;
    println!("  i32::MAX + 1 wrapping = {}", max.wrapping_add(1));

    // saturating_add：饱和到边界
    println!("  i32::MAX + 1 saturating = {}", max.saturating_add(1));

    section("浮点取模（C 不支持）");
    let x = 1234.567_f64;
    println!("  {x} % 10 = {}", x % 10.0);

    section("按位运算 + 取反用 !");
    let hi: u8 = 0b1110_0000;
    let lo: u8 = 0b0000_1111;
    println!("  hi      = {hi:#010b}");
    println!("  lo      = {lo:#010b}");
    println!("  hi & lo = {:08b}", hi & lo);
    println!("  hi | lo = {:08b}", hi | lo);
    println!("  hi ^ lo = {:08b}", hi ^ lo);
    println!("  !hi     = {:08b}", !hi); // 注意是 ! 不是 ~
    println!("  hi << 1 = {:08b}", hi << 1);
    println!("  hi >> 1 = {:08b}", hi >> 1);

    section("比较：两侧类型必须相同");
    let n: u8 = 5;
    let m: u8 = 10;
    println!("  {n} <  {m} : {}", n < m);
    println!("  {n} == {m} : {}", n == m);
    println!("  {n} != {m} : {}", n != m);

    section("按位优先级 > 比较（与 C 相反，更直觉）");
    // Rust: `n & BIT != 0` 解释为 `(n & BIT) != 0` —— 正是我们想要的
    let flags: u8 = 0b0000_0100;
    let bit: u8 = 0b0000_0100;
    println!("  flags & bit != 0 = {}", flags & bit != 0); // true

    section("逻辑 && / ||（短路求值）");
    // 用返回 bool 的函数 + 外部 mut 状态演示副作用是否被触发
    let mut side_effect_count = 0u32;
    fn f(count: &mut u32) -> bool {
        *count += 1;
        true
    }
    let left_false = false;
    let _ = left_false && f(&mut side_effect_count);
    println!("  false && f() → 副作用触发次数 = {side_effect_count}");

    side_effect_count = 0;
    let left_true = true;
    let _ = left_true || f(&mut side_effect_count);
    println!("  true  || f() → 副作用触发次数 = {side_effect_count}");

    section("复合赋值（没有 ++ 和链式赋值）");
    let mut total = 0;
    total += 5;
    total *= 2;
    total <<= 1;
    println!("  total = {total}");
    // ❌ 没有自增：total++;
    // ❌ 不能链式赋值：let x = let y = 3;
}
