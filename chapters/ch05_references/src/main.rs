//! 第5章 引用 —— 入口。
//!
//! 章节示例：
//! - `01_value_references`   —— 共享引用 `&T` / 可变引用 `&mut T`，按值 vs 按引用
//! - `02_using_references`   —— `&`/`*`、隐式解引用、引用赋值、引用的引用、比较、胖指针
//! - `03_lifetimes_basics`   —— 生命周期、借用局部变量、悬空引用为何被编译器拒绝
//! - `04_lifetimes_in_fn`    —— 函数签名中的生命周期参数、`'static`、返回引用、省略规则
//! - `05_lifetimes_in_struct`—— 包含引用的结构体、多个独立生命周期参数
//! - `06_shared_vs_mutable`  —— 共享 vs 可变规则、reborrow、自赋值陷阱、Rust vs C 的 const

fn main() {
    println!("第5章 引用");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch05_references --example 01_value_references");
    println!("  cargo run -p ch05_references --example 02_using_references");
    println!("  cargo run -p ch05_references --example 03_lifetimes_basics");
    println!("  cargo run -p ch05_references --example 04_lifetimes_in_fn");
    println!("  cargo run -p ch05_references --example 05_lifetimes_in_struct");
    println!("  cargo run -p ch05_references --example 06_shared_vs_mutable");
}
