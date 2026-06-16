//! 第4章 所有权与移动 —— 入口。
//!
//! 章节示例：
//! - `01_ownership`             —— 所有权树：变量拥有值，结构体/Vec 拥有字段/元素
//! - `02_moves`                 —— 移动语义：赋值、传参、返回值都是 move
//! - `03_moves_in_collections`  —— 从 Vec 索引中移出值、控制流中的移动
//! - `04_copy_types`            —— `Copy` 类型：移动的例外
//! - `05_rc_arc`                —— `Rc` / `Arc` 共享所有权

fn main() {
    println!("第4章 所有权与移动");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch04_ownership --example 01_ownership");
    println!("  cargo run -p ch04_ownership --example 02_moves");
    println!("  cargo run -p ch04_ownership --example 03_moves_in_collections");
    println!("  cargo run -p ch04_ownership --example 04_copy_types");
    println!("  cargo run -p ch04_ownership --example 05_rc_arc");
}
