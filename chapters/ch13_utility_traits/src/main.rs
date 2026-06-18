//! 第13章 实用特性 —— 入口。
//!
//! 章节示例：
//! - `01_drop`        —— Drop 析构：栈式析构顺序、手动 drop、早退
//! - `02_sized`       —— Sized / ?Sized：编译期大小、胖指针、dyn 与切片
//! - `03_clone_copy`  —— Clone vs Copy：深拷贝、位拷贝、为何 String 不能 Copy
//! - `04_deref`       —— Deref / DerefMut：解引用、自动解引用、newtype 转发
//! - `05_default`     —— Default：零值构造、结构体更新语法、泛型工厂
//! - `06_conversions` —— From/Into、AsRef/AsMut、Borrow、TryFrom、Cow

fn main() {
    println!("第13章 实用特性");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch13_utility_traits --example 01_drop");
    println!("  cargo run -p ch13_utility_traits --example 02_sized");
    println!("  cargo run -p ch13_utility_traits --example 03_clone_copy");
    println!("  cargo run -p ch13_utility_traits --example 04_deref");
    println!("  cargo run -p ch13_utility_traits --example 05_default");
    println!("  cargo run -p ch13_utility_traits --example 06_conversions");
}
