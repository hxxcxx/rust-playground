//! 第11章 特性与泛型 —— 入口。
//!
//! 章节示例：
//! - `01_trait_objects`       —— trait object / `dyn Trait` / 动态分发
//! - `02_defining_traits`     —— 定义、实现 trait；默认方法；derive 派生
//! - `03_orphan_and_supertraits` —— 孤儿规则、超特性、完全限定方法调用
//! - `04_generic_functions`   —— 泛型函数、trait bound、`where` 子句、多重 bound
//! - `05_impl_trait`          —— `impl Trait` 参数与返回值；静态 vs 动态分发对比
//! - `06_associated_items`    —— 关联类型 / 关联常量 / 关联函数（Iterator 风格）

fn main() {
    println!("第11章 特性与泛型");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch11_traits_and_generics --example 01_trait_objects");
    println!("  cargo run -p ch11_traits_and_generics --example 02_defining_traits");
    println!("  cargo run -p ch11_traits_and_generics --example 03_orphan_and_supertraits");
    println!("  cargo run -p ch11_traits_and_generics --example 04_generic_functions");
    println!("  cargo run -p ch11_traits_and_generics --example 05_impl_trait");
    println!("  cargo run -p ch11_traits_and_generics --example 06_associated_items");
}
