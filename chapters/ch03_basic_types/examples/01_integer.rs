//! 3.1 整数类型：字面量、字节字面量、`as` 转换、常用方法
//!
//! 运行：`cargo run -p ch03_basic_types --example 01_integer`

use ch03_basic_types::section;

fn main() {
    section("带类型后缀的字面量");
    let a: i8 = 116i8;
    let b: u32 = 0xcafe_u32; // 十六进制
    let c = 0b0010_1010_i32; // 二进制，下划线分隔
    let d = 0o106_u16; // 八进制
    println!("i8 = {a}, u32(hex) = {b}, i32(bin) = {c}, u16(oct) = {d}");

    section("字节字面量 b'X'");
    let byte: u8 = b'A';
    assert_eq!(byte, 65); // 'A' 的 ASCII 码是 65
    assert_eq!(b'\x1b', 27); // ESC 字符
    assert_eq!(b'\\', 92);
    println!("b'A' = {byte}, b'\\x1b' = {}, b'\\\\' = {}", b'\x1b', b'\\');

    section("`as` 类型转换");
    // 在范围内：直接保留数值
    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);
    // 符号扩展 / 零扩展
    assert_eq!((-1_i16) as i32, -1_i32); // 负数符号扩展
    assert_eq!(65535_u16 as i32, 65535_i32); // 无符号零扩展
    // 超出范围：截断为对 2^N 取模
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);
    assert_eq!((-1_i8) as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);
    println!("所有转换均符合预期（见 assert！）");

    section("整数的内置方法");
    assert_eq!(2_u16.pow(4), 16); // 求幂
    assert_eq!((-4_i32).abs(), 4); // 绝对值
    assert_eq!(0b101101_u8.count_ones(), 4); // 二进制中 1 的个数
    println!(
        "2^4 = {}, |-4| = {}, popcount(0b101101) = {}",
        2_u16.pow(4),
        (-4_i32).abs(),
        0b101101_u8.count_ones()
    );

    section("方法调用优先级陷阱");
    // 方法调用优先级高于一元前缀运算符！
    // 下面是正确写法：用括号包住负值
    assert_eq!((-4_i32).abs(), 4);
    // 错误写法：-4_i32.abs() 会先求 4.abs()=4，再取负 → -4
    assert_eq!(-4_i32.abs(), -4);
    println!(
        "(-4_i32).abs() = {}, -4_i32.abs() = {}",
        (-4_i32).abs(),
        -4_i32.abs()
    );
}
