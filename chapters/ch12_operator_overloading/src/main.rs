//! 第12章 运算符重载 —— 入口。
//!
//! 章节示例：
//! - `01_arithmetic`     —— 算术运算符 Add/Sub/Mul/Div/Rem（复数 +）
//! - `02_unary_assign`   —— 一元 Neg + 复合赋值 AddAssign/SubAssign
//! - `03_generic_rhs`   —— Rhs 泛型参数：Vec * 标量、不同类型相加
//! - `04_bitwise`        —— BitAnd/BitOr/BitXor/Not/Shl/Shr
//! - `05_equality_order` —— PartialEq / PartialOrd / Hash / derive
//! - `06_indexing`       —— Index / IndexMut：自定义下标访问

fn main() {
    println!("第12章 运算符重载");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch12_operator_overloading --example 01_arithmetic");
    println!("  cargo run -p ch12_operator_overloading --example 02_unary_assign");
    println!("  cargo run -p ch12_operator_overloading --example 03_generic_rhs");
    println!("  cargo run -p ch12_operator_overloading --example 04_bitwise");
    println!("  cargo run -p ch12_operator_overloading --example 05_equality_order");
    println!("  cargo run -p ch12_operator_overloading --example 06_indexing");
}
