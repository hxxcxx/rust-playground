//! 9.5 #[derive] 派生常用 trait
//!
//! 关键结论：
//! - `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]` 自动实现常用 trait。
//! - 前提：所有字段都必须实现对应 trait（递归检查）。
//! - 派生的 trait 会成为类型的「公共 API」一部分 —— 应该慎重选择。
//! - 比较类的 PartialEq/Eq/PartialOrd/Ord 不是必须派生的（取决于语义）。
//! - 详细见第 13 章。
//!
//! 运行：`cargo run -p ch09_structs --example 05_derive_traits`

use ch09_structs::section;

/// 一个普通二维点：派生多个 trait。
/// 注意：f64 不实现 Hash（因为 NaN != NaN），所以不能 derive Hash。
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

/// 颜色 RGB：派生 Eq（因为 u8 是 Eq）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

/// 员工：故意不派生 PartialOrd —— 因为「比较两个员工」语义上没意义。
#[derive(Debug, Clone, PartialEq)]
struct Employee {
    id: u32,
    name: String,
}

fn main() {
    section("Debug：用 {:?} 打印技术细节");
    let p = Point { x: 1.5, y: 2.5 };
    println!("  {:?} / {p:#?}（多行美化）", p);

    section("Clone + Copy：值类型可以随意拷贝");
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = p1; // Copy —— p1 仍然可用
    // Copy 类型其实不需要显式 clone —— 直接赋值就拷贝了
    let p3 = p1;
    println!("  p1 = {p1:?}, p2 = {p2:?}, p3 = {p3:?}");

    section("PartialEq：用 == 和 != 比较");
    let c1 = Color {
        r: 255,
        g: 128,
        b: 0,
    };
    let c2 = Color {
        r: 255,
        g: 128,
        b: 0,
    };
    println!("  c1 == c2 ? {}", c1 == c2);

    section("Hash：可作为 HashMap 的 Key");
    use std::collections::HashMap;
    let mut palette: HashMap<Color, &str> = HashMap::new();
    palette.insert(Color { r: 255, g: 0, b: 0 }, "red");
    palette.insert(Color { r: 0, g: 255, b: 0 }, "green");
    println!("  palette 大小 = {}", palette.len());
    println!("  red = {:?}", palette.get(&Color { r: 255, g: 0, b: 0 }));

    section("不派生 PartialOrd —— Employee 之间不比较大小");
    let e1 = Employee {
        id: 1,
        name: "Alice".into(),
    };
    let e2 = Employee {
        id: 2,
        name: "Bob".into(),
    };
    println!("  e1 == e2 ? {}", e1 == e2);
    // e1 < e2; // ❌ 编译错误：Employee 没有实现 PartialOrd
    println!("  → 故意不派生 PartialOrd，避免无意义的大小比较");

    section("派生清单建议（生产代码常用组合）");
    println!("  数据值类型  : Debug, Clone, Copy, PartialEq, Eq, Hash");
    println!("  含 String 等: Debug, Clone, PartialEq（不能 Copy）");
    println!("  排序语义类型: + PartialOrd, Ord");
}
