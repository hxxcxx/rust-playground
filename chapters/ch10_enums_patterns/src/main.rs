//! 第10章 枚举和模式 —— 入口。
//!
//! 章节示例：
//! - `01_c_style_enums`     —— C 风格枚举 + 整数值 + 转整数（不允许反向）
//! - `02_enums_with_data`   —— 含数据变体：元组变体 / 结构体变体 / Json 枚举
//! - `03_generic_enums`     —— 泛型枚举 Option/Result + BinaryTree 二叉树
//! - `04_patterns_basics`   —— 字面量/变量/通配符/范围/元组/结构体 模式
//! - `05_advanced_patterns` —— 引用模式 ref/& + 守卫 + 多模式 | + @绑定
//! - `06_pattern_uses`      —— let/for/函数参数/if let/while let 中的模式

fn main() {
    println!("第10章 枚举和模式");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch10_enums_patterns --example 01_c_style_enums");
    println!("  cargo run -p ch10_enums_patterns --example 02_enums_with_data");
    println!("  cargo run -p ch10_enums_patterns --example 03_generic_enums");
    println!("  cargo run -p ch10_enums_patterns --example 04_patterns_basics");
    println!("  cargo run -p ch10_enums_patterns --example 05_advanced_patterns");
    println!("  cargo run -p ch10_enums_patterns --example 06_pattern_uses");
}
