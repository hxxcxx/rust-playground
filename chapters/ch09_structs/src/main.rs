//! 第9章 结构体 —— 入口。
//!
//! 章节示例：
//! - `01_named_field_structs`  —— 具名字段结构体 + 字段简写 + `..` 展开
//! - `02_tuple_unit_structs`   —— 元组结构体 + 类单元结构体 + Newtype
//! - `03_methods`             —— impl 块、self/&self/&mut self、类型关联函数
//! - `04_generics_lifetimes`  —— 泛型结构体 + 带生命周期参数的结构体
//! - `05_derive_traits`       —— #[derive] 派生 Debug/Clone/Copy/PartialEq 等
//! - `06_interior_mutability` —— Cell + RefCell：在不可变外壳下做可变

fn main() {
    println!("第9章 结构体");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch09_structs --example 01_named_field_structs");
    println!("  cargo run -p ch09_structs --example 02_tuple_unit_structs");
    println!("  cargo run -p ch09_structs --example 03_methods");
    println!("  cargo run -p ch09_structs --example 04_generics_lifetimes");
    println!("  cargo run -p ch09_structs --example 05_derive_traits");
    println!("  cargo run -p ch09_structs --example 06_interior_mutability");
}
