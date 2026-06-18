//! 第6章 表达式 —— 入口。
//!
//! 章节示例：
//! - `01_block_semicolon`   —— 代码块与分号：Rust 作为「表达式语言」的核心
//! - `02_if_match`          —— `if`/`else if`/`else` 与 `match` 都是表达式
//! - `03_if_let_while_let`  —— `if let` / `while let`：单分支模式匹配的简写
//! - `04_loops`             —— `while` / `for` / `loop` 三种循环 + 区间
//! - `05_break_continue`    —— `break` / `continue` 与「循环标签」
//! - `06_operators`         —— 算术 / 按位 / 比较 / 逻辑运算符
//! - `07_casting`           —— `as` 类型转换 + 解引用强制转换
//! - `08_closures`          —— 闭包：轻量级函数值

fn main() {
    println!("第6章 表达式");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch06_expressions --example 01_block_semicolon");
    println!("  cargo run -p ch06_expressions --example 02_if_match");
    println!("  cargo run -p ch06_expressions --example 03_if_let_while_let");
    println!("  cargo run -p ch06_expressions --example 04_loops");
    println!("  cargo run -p ch06_expressions --example 05_break_continue");
    println!("  cargo run -p ch06_expressions --example 06_operators");
    println!("  cargo run -p ch06_expressions --example 07_casting");
    println!("  cargo run -p ch06_expressions --example 08_closures");
}
