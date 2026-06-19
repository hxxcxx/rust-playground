//! 第15章 迭代器 —— 入口。
//!
//! 章节示例：
//! - `01_basics`         —— Iterator trait / next / for / 三种 iter
//! - `02_adapters`       —— 适配器：map/filter/take/skip/zip/enumerate/flat_map
//! - `03_consumers`      —— 消费者：collect/sum/count/fold/any/all/for_each
//! - `04_custom`         —— 自定义迭代器：实现 Iterator（Countdown/Evens/树）
//! - `05_chains`         —— 迭代器链实战 + 惰性求值 + collect 类型推断
//! - `06_strings_ranges` —— 字符串/Range/HashMap 迭代 + 性能对比

fn main() {
    println!("第15章 迭代器");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch15_iterators --example 01_basics");
    println!("  cargo run -p ch15_iterators --example 02_adapters");
    println!("  cargo run -p ch15_iterators --example 03_consumers");
    println!("  cargo run -p ch15_iterators --example 04_custom");
    println!("  cargo run -p ch15_iterators --example 05_chains");
    println!("  cargo run -p ch15_iterators --example 06_strings_ranges");
}
