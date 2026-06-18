//! 6.2 `if` 与 `match` 表达式
//!
//! 关键结论：
//! - `if cond { ... } else { ... }`：条件必须是精确 `bool`（Rust 不隐式数字→bool）。
//! - 不需要括号；花括号是必需的。
//! - `match` 是 C `switch` 的强化版：必须穷尽所有情况，模式可带守卫。
//! - `if`/`match` 所有分支必须返回相同类型。
//! - `_` 是通配模式，类似 `default:`，必须放最后。
//!
//! 运行：`cargo run -p ch06_expressions --example 02_if_match`

use ch06_expressions::section;

fn main() {
    section("`if` / `else if` / `else` 都是表达式");
    let n = 7;
    let kind = if n == 0 {
        "zero"
    } else if n < 0 {
        "negative"
    } else {
        "positive"
    };
    println!("n={n} is {kind}");

    section("没有 else 的 if 必须返回 `()`");
    // 没有 else 的 if 表达式默认 else 分支为 `()`,
    // 因此两侧类型必须一致为 `()`。
    if n > 0 {
        println!("  n > 0 分支副作用");
    }

    section("match 整数（类似 C 的 switch）");
    let http_status = 404;
    let msg = match http_status {
        200 => "OK",
        304 => "Not Modified",
        404 => "Not Found",
        _ => "unknown", // 通配符必须放最后
    };
    println!("  HTTP {http_status} → {msg}");

    section("match 枚举：必须穷尽变体");
    let color = std::fmt::Alignment::Center;
    let desc = match color {
        std::fmt::Alignment::Left => "←",
        std::fmt::Alignment::Right => "→",
        std::fmt::Alignment::Center => "↔",
    };
    println!("  alignment = {desc}");

    section("match 解构 Option");
    let maybe_name: Option<&str> = Some("Ada");
    let greeting = match maybe_name {
        Some(name) => format!("Hello, {name}!"),
        None => "Greetings, stranger.".to_string(),
    };
    println!("  {greeting}");

    section("match 可以一次解包元组");
    let point: (i32, i32) = (3, -2);
    let location: String = match point {
        (0, 0) => "origin".to_string(),
        (x, 0) => format!("on x-axis at {x}"),
        (0, y) => format!("on y-axis at {y}"),
        (x, y) if x > 0 && y > 0 => format!("quadrant I ({x},{y})"),
        (x, y) => format!("elsewhere ({x},{y})"),
    };
    println!("  {location}");

    section("所有分支必须返回同一类型（编译期检查）");
    // ❌ 编译错误：分支类型不一致
    // let bad = match n { 0 => "zero", _ => 9 }; // &str vs 整数
    let ok = match n {
        0 => "zero",
        _ => "nonzero",
    };
    println!("  ok = {ok}");
}
