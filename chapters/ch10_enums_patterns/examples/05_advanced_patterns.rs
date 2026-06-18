//! 10.5 高级模式：引用 ref/& + 守卫 + 多模式 + @ 绑定
//!
//! 关键结论：
//! - `ref name` / `ref mut name`：在模式里借用（不移动值）。
//! - `&pattern`：匹配引用本身（解引用一层）。
//! - `pattern if condition`：匹配守卫（match 专属，let 中不能用）。
//! - `pat1 | pat2 | pat3`：多重模式（同一分支匹配多个）。
//! - `name @ pattern`：绑定整个匹配值到一个变量（同时仍按模式解构）。
//!
//! 运行：`cargo run -p ch10_enums_patterns --example 05_advanced_patterns`

use ch10_enums_patterns::{Json, section};

fn main() {
    section("ref pattern：借用而非移动");
    let account = Account {
        name: "Alice".to_string(),
        language: "Rust".to_string(),
    };
    // 用 ref 借用，account 不会被消耗，后面还能用
    #[allow(clippy::match_single_binding)]
    match account {
        Account {
            ref name,
            ref language,
            ..
        } => {
            println!("  name={name}, language={language}");
        }
    }

    section("ref mut：借用可变引用");
    let mut result: Result<String, String> = Ok("hello".into());
    match result {
        // 用 ref mut 借用，使 result 本身不被消耗
        Ok(ref mut line) => {
            line.push_str(" world");
            println!("  Ok 内就地修改: {line}");
        }
        Err(ref err) => println!("  Err: {err}"),
    }

    section("& pattern：匹配引用本身");
    let value: &i32 = &42;
    match value {
        &n => println!("  解出引用指向的值: {n}"),
    }
    // 嵌套：匹配 &Option<&char>
    let maybe_char: Option<&char> = Some(&'A');
    match maybe_char {
        Some(&c) => println!("  Some(&'A') 解出 char: {c}"),
        None => println!("  None"),
    }

    section("匹配守卫 (match guard)");
    let point: (i32, i32) = (3, 4);
    let quadrant = match point {
        (x, y) if x > 0 && y > 0 => "I",
        (x, y) if x < 0 && y > 0 => "II",
        (x, y) if x < 0 && y < 0 => "III",
        (x, y) if x > 0 && y < 0 => "IV",
        _ => "on axis",
    };
    println!("  {:?} 在象限 {quadrant}", point);

    section("多重模式 |");
    let c = ' ';
    let kind = match c {
        ' ' | '\t' | '\n' | '\r' => "whitespace",
        'a'..='z' | 'A'..='Z' => "letter",
        '0'..='9' => "digit",
        _ => "other",
    };
    println!("  '{c}' → {kind}");

    section("@ 绑定：同时匹配 + 捕获整个值");
    let n: i32 = 75;
    match n {
        // 把 0..=99 范围内的值绑到变量 bound
        bound @ 0..=99 => println!("  小数: {bound}"),
        bound @ 100..=999 => println!("  中数: {bound}"),
        n => println!("  大数: {n}"),
    }

    section("@ 绑定 + 枚举变体");
    let json = Json::Number(42.0);
    match json {
        // 把整个 Json::Number 值绑到 num_val，同时验证内部值
        num_val @ Json::Number(n) if n >= 0.0 => {
            println!("  非负数: {:?} (内部 = {n})", num_val);
        }
        Json::Number(n) => println!("  负数: {n}"),
        _ => println!("  非 Number"),
    }

    section("ref + @ 组合");
    let s = String::from("hello");
    // 演示 ref 在 match 中的用法（clippy 会建议 let；教学保留）
    #[allow(clippy::match_single_binding)]
    match s {
        // 借用 s 同时匹配其长度 —— 但 String 不是模式可直接匹配的，这里只演示 ref
        ref borrowed => println!("  借用: {borrowed}"),
    }
}

#[derive(Debug)]
struct Account {
    name: String,
    language: String,
}
