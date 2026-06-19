//! 17.1 String vs &str —— 拥有 vs 借用
//!
//! 关键结论：
//! - `String`：堆分配、可增长、拥有所有权的 UTF-8 字符串。
//! - `&str`：字符串切片，胖指针（指针 + 字节长度），借用某处的 UTF-8 数据。
//! - `&str` 可以指向：String 的内容、字符串字面量（静态区）、Box<str> 等。
//! - 字符串字面量 `"hi"` 的类型是 `&'static str`（编译期已知，活在程序整个生命周期）。
//! - 互转：
//!   * String → &str：自动「deref 强制转换」（&s 自动变 &str）。
//!   * &str → String：`s.to_string()` / `String::from(s)` / `s.to_owned()` / `s.into()`。
//! - 函数参数优先用 `&str`（更通用，能接受 &String 和 &str）。
//!
//! 运行：`cargo run -p ch17_strings_text --example 01_string_str`

use ch17_strings_text::section;

fn main() {
    section("String：拥有、可变、堆分配");
    let mut s = String::from("hello");
    s.push_str(", world");
    s.push('!');
    println!("  {s}");
    println!("  len = {} 字节, 容量 = {}", s.len(), s.capacity());

    section("&str：借用、不可变、胖指针");
    let literal: &str = "世界"; // &'static str，存在静态区
    let slice: &str = &s[0..5]; // 借用 String 的一部分
    println!("  字面量: {literal}");
    println!("  切片: {slice}");

    section("&str → String：四种等价写法");
    let s1 = "rust".to_string();
    let s2 = String::from("rust");
    let s3: String = "rust".into();
    let s4 = "rust".to_owned();
    println!("  to_string: {s1:?}");
    println!("  from:      {s2:?}");
    println!("  into:      {s3:?}");
    println!("  to_owned:  {s4:?}");

    section("String → &str：自动 deref");
    let owned = String::from("hello");
    takes_str(&owned); // &String 自动转 &str
    takes_str("world"); // 字面量本身就是 &str

    section("函数参数：优先 &str（更通用）");
    // 接受 &str 的函数能同时处理 &String 和 &str。
    let owned = String::from("abc");
    println!("  len_of(&String) = {}", len_of(&owned));
    println!("  len_of(&str)    = {}", len_of("hello"));

    section("String 的所有权语义");
    let s = String::from("data");
    let _borrow: &str = &s; // 借用
    consume(s); // 移动
    // println!("{s}"); // ❌ s 已被移动
    println!("  （s 已被 consume 消耗）");

    section("&str 可以指向不同来源");
    let owned = String::from("owned");
    let r1: &str = &owned; // 指向 String 的堆数据
    let r2: &str = "static"; // 指向静态区
    let boxed: Box<str> = "boxed".into();
    let r3: &str = &boxed; // 指向 Box<str>
    println!("  {r1} / {r2} / {r3}");

    section("字符串字面量是 &'static str");
    static_global();
}

/// 接受 &str 的函数（最通用的字符串参数形式）。
fn takes_str(s: &str) {
    println!("  takes_str: {s}");
}

/// 演示 &str 参数能接受多种来源。
fn len_of(s: &str) -> usize {
    s.len()
}

/// 消耗 String。
fn consume(_s: String) {}

/// 演示字符串字面量的 'static 生命周期。
fn static_global() {
    let s: &'static str = "我活到程序结束";
    // 可以返回 'static 引用，因为它在静态区。
    println!("  static 字面量: {s}");
}
