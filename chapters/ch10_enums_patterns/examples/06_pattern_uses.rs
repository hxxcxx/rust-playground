//! 10.6 模式的使用场景：let / for / 函数参数 / if let / while let
//!
//! 关键结论：
//! - 模式不仅用在 match 分支里，也出现在：
//!   * `let PAT = expr;` —— 解构赋值（必须用「不可反驳模式」）
//!   * `fn f(PAT: T)` —— 函数参数解构
//!   * `for PAT in iter` —— 循环变量解构
//!   * `if let PAT = expr` / `while let PAT = expr` —— 用「可反驳模式」
//! - 不可反驳模式（irrefutable）：保证一定能匹配（如 `(x, y)` 对元组）。
//! - 可反驳模式（refutable）：可能不匹配（如 `Some(x)` 对 Option）。
//! - let/for/函数参数 必须用不可反驳；if let/while let 可以用可反驳。
//!
//! 运行：`cargo run -p ch10_enums_patterns --example 06_pattern_uses`

use ch10_enums_patterns::{Json, section};
use std::collections::HashMap;

/// 演示「函数参数解构」：参数本身是一个模式。
fn distance_to((x, y): (f64, f64)) -> f64 {
    (x * x + y * y).sqrt()
}

#[derive(Debug)]
struct Track {
    album: String,
    track_number: u32,
    title: String,
    year: u32,
}

fn main() {
    section("let 解构结构体（不可反驳模式）");
    let song = Track {
        album: "Lateralus".into(),
        track_number: 5,
        title: "Schism".into(),
        year: 2001,
    };
    let Track {
        album,
        track_number,
        title,
        year: _,
    } = song;
    println!("  {album} #{track_number} - {title}");

    section("let 解构元组");
    let coords = (3.0, 4.0);
    let (x, y) = coords;
    println!("  x={x}, y={y}");

    section("函数参数解构");
    let d = distance_to((3.0, 4.0));
    println!("  distance_to((3,4)) = {d:.2}");

    section("for 解构 HashMap 的 (key, value)");
    let mut cache: HashMap<u32, &str> = HashMap::new();
    cache.insert(1, "Alice");
    cache.insert(2, "Bob");
    for (id, name) in &cache {
        println!("  #{id}: {name}");
    }

    section("for 解构嵌套元组");
    let pairs = [(1, "a"), (2, "b"), (3, "c")];
    for (n, s) in pairs {
        println!("  {n} → {s}");
    }

    section("if let：单分支匹配（可反驳模式）");
    let maybe_num: Option<i32> = Some(42);
    if let Some(n) = maybe_num {
        println!("  拿到值: {n}");
    } else {
        println!("  是 None");
    }

    section("while let：循环直到不匹配");
    let mut stack = vec![1, 2, 3];
    let mut sum = 0;
    while let Some(top) = stack.pop() {
        sum += top;
    }
    println!("  累加结果: {sum}");

    section("let-else：Rust 1.65+ 的早期返回语法");
    fn parse_num(s: &str) -> Result<i32, &'static str> {
        let n: i32 = s.parse().map_err(|_| "parse failed")?;
        Ok(n)
    }
    let Ok(n) = parse_num("42") else {
        println!("  解析失败");
        return;
    };
    println!("  let-else 解析得到: {n}");

    section("闭包参数也能用模式（解引用）");
    let nums = [1, 2, 3, 4, 5];
    // |&num| 解引用模式：把 &i32 解成 i32
    let doubled: Vec<i32> = nums.iter().map(|&num| num * 2).collect();
    println!("  doubled = {doubled:?}");

    section("递归遍历 Json（实战综合）");
    let mut obj = HashMap::new();
    obj.insert("name".into(), Json::String("Alice".into()));
    obj.insert("age".into(), Json::Number(30.0));
    obj.insert(
        "tags".into(),
        Json::Array(vec![Json::String("rust".into()), Json::Boolean(true)]),
    );
    let json = Json::Object(Box::new(obj));
    println!("  所有字符串值：");
    collect_strings(&json, 1);
}

/// 递归收集 Json 中的所有 String 值。
fn collect_strings(json: &Json, indent: usize) {
    let pad = "  ".repeat(indent);
    match json {
        Json::Null => println!("{pad}(null)"),
        Json::Boolean(b) => println!("{pad}{b}"),
        Json::Number(n) => println!("{pad}{n}"),
        Json::String(s) => println!("{pad}\"{s}\""),
        Json::Array(items) => {
            println!("{pad}[");
            for item in items {
                collect_strings(item, indent + 1);
            }
            println!("{pad}]");
        }
        Json::Object(map) => {
            println!("{pad}{{");
            for (k, v) in map.iter() {
                println!("{pad}  {k}:");
                collect_strings(v, indent + 2);
            }
            println!("{pad}}}");
        }
    }
}
