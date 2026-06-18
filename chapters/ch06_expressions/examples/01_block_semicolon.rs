//! 6.1 代码块与分号：Rust 作为「表达式语言」
//!
//! 关键结论：
//! - C 里 `if`/`for` 是「语句」、不产出值；Rust 里它们是「表达式」，会产出值。
//! - `{ ... }` 是块表达式，其值 = 块最后一条「不带分号」的表达式。
//! - 末尾加 `;` → 该块返回 `()`（即丢弃值）；省略 `;` → 返回该表达式值。
//! - 这是 Rust 没有三元运算符 `?:` 的原因 —— `if` 表达式已经能担当一切。
//! - `let` 声明本身就是「语句」（不是表达式），所以末尾必须加分号。
//!
//! 运行：`cargo run -p ch06_expressions --example 01_block_semicolon`

use ch06_expressions::section;

fn main() {
    section("C 是语句导向，Rust 是表达式导向");

    // C 写法: int status = cond ? HTTP_OK : HTTP_ERR;
    // Rust: 直接用 `if` 表达式赋值（没有 ?: 运算符）。
    let temp: i32 = 38;
    let status: &str = if temp <= 40 { "ok" } else { "server melted" };
    println!("temp={temp} → status={status}");

    section("块表达式：最后的「无分号表达式」是块的返回值");

    let display_name: String = match author_id() {
        Some(name) => name,
        None => {
            // 块里可以放任意多语句……
            let net = "guest";
            let ip = format!("{net}@unknown");
            // ……但块值由最后一条「无分号」表达式决定：
            ip // ← 没有分号，所以这个 String 成为块值
        }
    };
    println!("display_name = {display_name}");

    section("分号「丢弃」值：相同代码、只差一个分号、返回类型就不同");

    let with_value: i32 = {
        let a = 10;
        a + 5 // 不带分号：块的值是 15
    };
    println!("with_value = {with_value}");

    // 注意：带分号版本 `a + 5;` 会被 clippy 标为「无副作用语句」。
    // 下面用「方法调用」演示：分号会丢弃方法的返回值。
    #[allow(clippy::let_unit_value)]
    let without_value: () = {
        let s = String::from("hi");
        s.len(); // ← 带分号：方法被调用但返回值被丢弃，块值是 `()`
    };
    println!("without_value = {without_value:?} (单元类型)");

    section("`let` 声明可以延后初始化");
    // ⚠️ clippy 会建议「就近初始化」(`needless_late_init`)，
    // 这里保留是为了演示 Rust「先声明、分支中赋值」的语义。
    #[allow(clippy::needless_late_init)]
    let name: &str;
    if temp <= 40 {
        name = "cold";
    } else {
        name = "hot";
    }
    // 注意：name 不是 mut —— 它只被初始化一次（不同分支二选一）。
    println!("name = {name}");

    section("变量遮蔽（shadowing）：新变量同名覆盖旧变量");
    // 在循环里非常常见：`for line in file.lines() { let line = line?; }`
    let line: Result<i32, &str> = Ok(42);
    #[allow(clippy::unnecessary_literal_unwrap)]
    let line: i32 = line.unwrap(); // 新 line 类型变了
    println!("shadowed line = {line}");
}

/// 模拟「从某个 ID 查作者名」，返回 `Option<String>`。
fn author_id() -> Option<String> {
    None
}
