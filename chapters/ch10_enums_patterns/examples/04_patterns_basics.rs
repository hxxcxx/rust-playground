//! 10.4 模式基础：字面量/变量/通配符/范围/元组/结构体/数组/切片
//!
//! 关键结论：
//! - 模式（pattern）出现在 `match`/`if let`/`while let`/`let`/`for`/函数参数 中。
//! - 字面量模式：`100`、`"name"`、`true`、`'A'`。
//! - 变量模式：`x`（绑定新变量）；`_`（通配符，丢弃）。
//! - 范围模式：`0..=100`、`'a'..='z'`。
//! - 元组模式：`(x, y)`、`(0, 0)`。
//! - 结构体模式：`Point { x, y }`、`Point { x: 0, .. }`。
//! - 数组模式（定长）：`[a, b, c]`。
//! - 切片模式（变长）：`[first, rest @ ..]`。
//!
//! 运行：`cargo run -p ch10_enums_patterns --example 04_patterns_basics`

use ch10_enums_patterns::section;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Account {
    id: u32,
    name: String,
    language: String,
    email: Option<String>,
}

fn main() {
    section("字面量模式：精确匹配值");
    let code = 404;
    let msg = match code {
        200 => "OK",
        304 => "Not Modified",
        404 => "Not Found",
        _ => "unknown",
    };
    println!("  HTTP {code} → {msg}");

    section("变量模式：绑定新局部变量");
    let rabbits = 3;
    match rabbits {
        0 => println!("  没有兔子"),
        1 => println!("  一只兔子"),
        n => println!("  有 {n} 只兔子"),
    }

    section("通配符 _：忽略值");
    let color = "red";
    let caption = match color {
        "red" => "Stop",
        "green" => "Go",
        _ => "Unknown",
    };
    println!("  {color} → {caption}");

    section("范围模式 ..= （闭区间）");
    let grade = 85;
    let level = match grade {
        0..=59 => "F",
        60..=69 => "D",
        70..=79 => "C",
        80..=89 => "B",
        90..=100 => "A",
        _ => "invalid",
    };
    println!("  {grade} → {level}");

    section("字符范围 + 多重匹配 |");
    let c = 'G';
    let kind = match c {
        'a'..='z' => "lowercase",
        'A'..='Z' => "uppercase",
        '0'..='9' => "digit",
        _ => "other",
    };
    println!("  '{c}' → {kind}");

    section("元组模式：一次解包多个值");
    let point: (i32, i32) = (3, -2);
    let loc: String = match point {
        (0, 0) => "origin".to_string(),
        (x, 0) => format!("on x-axis at {x}"),
        (0, y) => format!("on y-axis at {y}"),
        (x, y) if x > 0 && y > 0 => format!("quadrant I ({x}, {y})"),
        (x, y) => format!("elsewhere ({x}, {y})"),
    };
    println!("  {loc}");

    section("结构体模式：解构字段");
    let p = Point { x: 30, y: 40 };
    match p {
        Point { x: 0, y: 0 } => println!("  at origin"),
        Point { x, y } => println!("  at ({x}, {y})"),
    }

    section("结构体简写 + .. 忽略其余字段");
    let acc = Account {
        id: 1,
        name: "Alice".into(),
        language: "Rust".into(),
        email: None,
    };
    // 演示 match 中解构结构体（虽然这里只有单分支，clippy 会建议用 let；
    // 实际生产代码会更复杂，会有多个分支）
    #[allow(clippy::match_single_binding)]
    match acc {
        Account { name, language, .. } => {
            println!("  Hi {name}, you write {language}");
        }
    }

    section("数组模式（定长）");
    let rgb: [u8; 3] = [255, 128, 0];
    match rgb {
        [r, g, b] => println!("  RGB = ({r}, {g}, {b})"),
    }

    section("切片模式（变长，用 .. 匹配任意长度）");
    greet_people(&["alice"]);
    greet_people(&["alice", "bob"]);
    greet_people(&["alice", "bob", "carol"]);
    greet_people(&[]);
}

fn greet_people(names: &[&str]) {
    match names {
        [] => println!("  Hello, nobody."),
        [a] => println!("  Hello, {a}."),
        [a, b] => println!("  Hello, {a} and {b}."),
        [a, .., b] => println!("  Hello, everyone from {a} to {b}."),
    }
}
