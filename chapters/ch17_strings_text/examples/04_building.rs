//! 17.4 构建字符串：push / extend / + / write! / format!
//!
//! 关键结论：
//! - 增量构建 String 的几种方式（按效率/可读性取舍）：
//!   * `push_str` / `push`：就地向末尾追加（常用）。
//!   * `+` / `+=`：拼接（消耗左操作数，产生新 String）。
//!   * `format!`：宏，最可读，但会分配多次。
//!   * `extend`：从迭代器批量追加。
//!   * `write!`：往 `&mut String`（实现 Write）写格式化内容。
//! - 大量拼接时，先用 `with_capacity` 预分配能避免反复扩容。
//! - `concat`/`join`：把迭代器/切片的多个字符串拼起来。
//!
//! 运行：`cargo run -p ch17_strings_text --example 04_building`

use ch17_strings_text::section;
use std::fmt::Write;

fn main() {
    section("push_str / push：就地追加");
    let mut s = String::new();
    s.push_str("hello");
    s.push(' ');
    s.push_str("world");
    println!("  {s}");

    section("with_capacity：预分配避免扩容");
    let mut s = String::with_capacity(100);
    for i in 0..10 {
        // write! 往 String 写格式化内容（String 实现了 fmt::Write）。
        let _ = write!(s, "[{i}] ");
    }
    println!("  {s}");

    section("+ 拼接：消耗左操作数");
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    let s3 = s1 + &s2; // s1 被消耗，s2 借用
    println!("  s1 + &s2 = {s3}");
    // s1 不能再用；s2 还能用。
    println!("  s2 还在: {s2}");

    section("+ 字面量（&str）");
    let s = String::from("hello") + " " + "world";
    println!("  String + &str + &str = {s}");

    section("format!：最可读，但分配多次");
    let name = "Alice";
    let age = 30;
    let s = format!("{name} 今年 {age} 岁");
    println!("  {s}");

    section("extend：从迭代器批量追加");
    let mut s = String::from("nums: ");
    s.extend([1, 2, 3].iter().map(|n| n.to_string()));
    println!("  {s}");

    section("join：用分隔符拼接");
    let parts = ["apple", "banana", "cherry"];
    let joined = parts.join(", ");
    println!("  join(\", \"): {joined}");

    section("concat：无分隔符拼接");
    let s = ["a", "b", "c"].concat();
    println!("  concat: {s}");

    section("迭代器 + collect 构建 String");
    let chars = ['h', 'i', '!'];
    let s: String = chars.iter().collect();
    println!("  chars → String: {s}");

    section("repeat：重复字符串");
    let s = "ab".repeat(3);
    println!("  \"ab\".repeat(3) = {s:?}");

    section("性能对比：循环 push_str vs format!");
    let n = 1000;
    // push_str（一次预分配）
    let t = std::time::Instant::now();
    let mut built = String::with_capacity(n * 3);
    for _ in 0..n {
        built.push_str("abc");
    }
    let push_time = t.elapsed();
    println!("  push_str ×{n}: {} 字节, {push_time:?}", built.len());

    // format!（每次分配）
    let t = std::time::Instant::now();
    let mut built = String::new();
    for _ in 0..n {
        // 这里用 write! 而非 format!（format! 会产生临时 String）。
        let _ = write!(built, "abc");
    }
    let write_time = t.elapsed();
    println!("  write! ×{n}: {} 字节, {write_time:?}", built.len());
    println!("  （大量拼接时，with_capacity + push_str 最快）");

    section("trim / replace / to_uppercase：常用变换");
    let s = "  Hello World  ".trim();
    println!("  trim: {s:?}");
    let s = "Hello".to_uppercase();
    println!("  to_uppercase: {s}");
    let s = "a-b-c".replace('-', "_");
    println!("  replace: {s}");
}
