//! 17.6 解析与转换：parse / FromStr / ToString
//!
//! 关键结论：
//! - `str::parse::<T>()`：把字符串解析成 T（需 T: FromStr），返回 Result。
//! - `FromStr` trait：自定义「字符串 → 类型」的解析逻辑。
//! - `ToString` / `Display`：把类型转成字符串。
//!   * 实现了 Display 自动获得 ToString（`.to_string()`）。
//! - `From<&str>` / `From<String>`：与 parse 类似但用于转换。
//! - 常用：数字解析、bool 解析、自定义格式解析。
//!
//! 运行：`cargo run -p ch17_strings_text --example 06_parse_convert`

use ch17_strings_text::section;
use std::str::FromStr;

fn main() {
    section("parse：字符串 → 数字");
    let n: i32 = "42".parse().unwrap();
    let f: f64 = "3.14".parse().unwrap();
    let b: bool = "true".parse().unwrap();
    println!("  parse i32: {n}");
    println!("  parse f64: {f}");
    println!("  parse bool: {b}");

    section("parse 用 turbofish 指定类型");
    let n = "255".parse::<u8>().unwrap();
    let hex = "ff".parse::<u32>(); // 默认按十进制，"ff" 解析失败
    println!("  parse::<u8>(\"255\"): {n}");
    println!("  parse::<u32>(\"ff\"): {:?}", hex.map(|_| "ok"));

    section("parse 失败处理（Result）");
    match "abc".parse::<i32>() {
        Ok(n) => println!("  解析成功: {n}"),
        Err(e) => println!("  解析失败: {e}"),
    }

    section("from_str_radix：按进制解析");
    let bin = i32::from_str_radix("1010", 2).unwrap(); // 二进制
    let hex = i32::from_str_radix("ff", 16).unwrap(); // 十六进制
    let oct = i32::from_str_radix("17", 8).unwrap(); // 八进制
    println!("  二进制 1010 → {bin}");
    println!("  十六进制 ff → {hex}");
    println!("  八进制 17 → {oct}");

    section("ToString / to_string：类型 → 字符串");
    let n = 42;
    let s1 = n.to_string(); // i32 实现了 Display → 自动有 ToString
    let s2 = true.to_string();
    let s3 = 2.5_f64.to_string();
    println!("  42.to_string(): {s1:?}");
    println!("  true.to_string(): {s2:?}");
    println!("  2.5.to_string(): {s3:?}");

    section("自定义 FromStr：解析「键=值」");
    match "host=localhost".parse::<KeyValue>() {
        Ok(kv) => println!("  解析: {} = {}", kv.key, kv.value),
        Err(e) => println!("  失败: {e}"),
    }
    match "no_equals_sign".parse::<KeyValue>() {
        Ok(kv) => println!("  {kv:?}"),
        Err(e) => println!("  预期失败: {e}"),
    }

    section("自定义 Display → 自动获得 ToString");
    let color = Color { r: 255, g: 128, b: 0 };
    let s = color.to_string(); // 因为实现了 Display
    println!("  to_string: {s}");

    section("批量解析：filter_map 跳过失败");
    let inputs = ["1", "2", "abc", "3", "xyz", "4"];
    let nums: Vec<i32> = inputs.iter().filter_map(|s| s.parse().ok()).collect();
    println!("  可解析的: {nums:?}");

    section("数字 → 字符串的进制转换");
    let n = 255_u32;
    println!("  255 二进制:    {:b}", n);
    println!("  255 十六进制:  {:x}", n);
    // 用 format! 得到字符串。
    let bin = format!("{:b}", n);
    let hex = format!("{:x}", n);
    println!("  format! 得到: bin={bin:?}, hex={hex:?}");

    section("String 与 &str 互转总结");
    let owned = String::from("hi");
    let borrowed: &str = &owned; // String → &str（自动）
    let back: String = borrowed.to_string(); // &str → String
    println!("  循环转换: {back:?}");
}

/// 键值对：演示自定义 FromStr。
#[derive(Debug)]
struct KeyValue {
    key: String,
    value: String,
}

impl FromStr for KeyValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('=') {
            Some((k, v)) => Ok(KeyValue {
                key: k.to_string(),
                value: v.to_string(),
            }),
            None => Err(format!("缺少 '=' 分隔符: {s:?}")),
        }
    }
}

/// 颜色：演示「实现 Display 后自动有 to_string」。
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
