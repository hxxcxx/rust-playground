//! 第7章 错误处理 —— 入口。
//!
//! 章节示例：
//! - `01_panic`           —— panic 的触发、展开栈、catch_unwind
//! - `02_result_basics`   —— Result 的方法：match / is_ok / unwrap_or / ok / err
//! - `03_propagate`       —— `?` 运算符：错误传播的语法糖
//! - `04_multiple_errors` —— 多种错误类型：GenericError / From 转换
//! - `05_custom_error`    —— 自定义错误类型：手动 impl + thiserror
//! - `06_main_returns`    —— main() 返回 Result 与错误处理策略

fn main() {
    println!("第7章 错误处理");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch07_errors --example 01_panic");
    println!("  cargo run -p ch07_errors --example 02_result_basics");
    println!("  cargo run -p ch07_errors --example 03_propagate");
    println!("  cargo run -p ch07_errors --example 04_multiple_errors");
    println!("  cargo run -p ch07_errors --example 05_custom_error");
    println!("  cargo run -p ch07_errors --example 06_main_returns");
}
