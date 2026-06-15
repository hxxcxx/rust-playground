//! 3.2 溢出处理：四种算术语义 `checked_*` / `wrapping_*` / `saturating_*` / `overflowing_*`
//!
//! 运行：`cargo run -p ch03_basic_types --example 02_overflow`

use ch03_basic_types::section;

fn main() {
    section("checked_*: 返回 Option，溢出则 None");
    assert_eq!(10_u8.checked_add(20), Some(30)); // 在范围内
    assert_eq!(100_u8.checked_add(200), None); // 300 > u8::MAX(255)
    assert_eq!((-128_i8).checked_div(-1), None); // i8::MIN / -1 无法表示 +128
    println!(
        "10 + 20 = {:?}, 100 + 200 = {:?}, (-128)/(-1) = {:?}",
        10_u8.checked_add(20),
        100_u8.checked_add(200),
        (-128_i8).checked_div(-1)
    );

    section("wrapping_*: 模 2^N，环绕");
    assert_eq!(100_u16.wrapping_mul(200), 20000); // 在范围内
    assert_eq!(500_u16.wrapping_mul(500), 53392); // 250000 % 65536 = 53392
    assert_eq!(500_i16.wrapping_mul(500), -12144); // 有符号环绕到负数
    assert_eq!(5_i16.wrapping_shl(17), 10); // 17 对 16 取模 = 1，左移 1 位
    println!(
        "500 * 500 (u16) = {}, (i16) = {}",
        500_u16.wrapping_mul(500),
        500_i16.wrapping_mul(500)
    );

    section("saturating_*: 钳制在 [MIN, MAX]");
    assert_eq!(32760_i16.saturating_add(10), 32767); // 钳到 i16::MAX
    assert_eq!((-32760_i16).saturating_sub(10), -32768); // 钳到 i16::MIN
    assert_eq!(255_u8.saturating_add(10), 255); // 钳到 u8::MAX
    println!(
        "32760 + 10 = {}, 255 + 10 = {}",
        32760_i16.saturating_add(10),
        255_u8.saturating_add(10)
    );

    section("overflowing_*: 返回 (结果, 是否溢出)");
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true)); // 环绕为 1，发生溢出
    // 移位：只有当移位距离 >= 类型位宽时，第二个返回值才为 true
    assert_eq!(5_u16.overflowing_shl(17), (10, true)); // 17 % 16 = 1
    assert_eq!(5_u16.overflowing_shl(2), (20, false)); // 正常左移
    println!(
        "255 + 2 = {:?}, 5 << 17 = {:?}",
        255_u8.overflowing_add(2),
        5_u16.overflowing_shl(17)
    );

    section("默认行为：debug 恐慌 / release 环绕");
    // 普通 + - * 在 debug 构建中溢出会 panic，在 release 中会环绕
    // 下面这个不会 panic（在范围内）：
    let _ok = 200_u8 + 50; // 250
    println!("普通运算 200 + 50 = {}", 200_u8 + 50);
    println!("\n提示：debug 中 200 + 100 会 panic；release 中会环绕成 44");
}
