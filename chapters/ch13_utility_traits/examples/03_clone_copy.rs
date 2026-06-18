//! 13.3 Clone 与 Copy —— 深拷贝 vs 位拷贝
//!
//! 关键结论：
//! - `Clone::clone(&self) -> Self`：显式深拷贝，可能昂贵（如 String 要复制堆数据）。
//!   调用方式：`x.clone()`。编译器「不会」自动 clone。
//! - `Copy`：标记 trait。实现 Copy 后，赋值/传参「自动按位复制」。
//!   要求：所有字段都是 Copy；与 Drop 互斥（Copy 暗示无需析构）。
//! - 实现 Copy 时通常也让 clone() 直接返回 `*self`（deriving 会自动做）。
//! - 为何 String 不能 Copy？因为它有堆分配，位拷贝会导致双重释放。
//!
//! 运行：`cargo run -p ch13_utility_traits --example 03_clone_copy`

use ch13_utility_traits::{Appellation, section};

fn main() {
    section("Copy 类型：赋值即复制");
    let a: i32 = 42;
    let b = a; // i32 是 Copy，这里复制了一份
    // a 和 b 都还能用（独立的副本）
    println!("  a = {a}, b = {b}");

    section("非 Copy 类型：赋值即「移动」");
    let s1 = String::from("hello");
    let s2 = s1; // String 不是 Copy —— 这里是「移动」，s1 失效
    // println!("{s1}"); // ❌ 编译错误：s1 已被移动
    println!("  s2 = {s2}（s1 已失效）");

    section("想保留原值？显式 clone()");
    let s3 = String::from("world");
    let s4 = s3.clone(); // 深拷贝：复制堆数据
    println!("  s3 = {s3}, s4 = {s4}（两个独立的 String）");

    section("Appellation：用 &'static str 让它变成 Copy");
    let a1 = Appellation { name: "Alice", nick: "Al" };
    let a2 = a1; // Copy！a1 还能用
    println!("  a1 = {a1}, a2 = {a2}");
    println!("  a1 还能用？ 是的，因为 Copy");

    section("derive Clone/Copy 的条件");
    // 所有字段都是 Copy → 整体可以 Copy。
    #[derive(Clone, Copy, Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1; // Copy
    println!("  p1 = {p1:?}, p2 = {p2:?}");

    section("含 String 的结构体：只能 Clone 不能 Copy");
    #[derive(Clone, Debug)]
    struct Person {
        name: String,
        age: u32,
    }
    let alice = Person { name: "Alice".into(), age: 30 };
    let bob = alice.clone(); // 必须 clone（不能 Copy，因为有 String）
    println!("  alice = {alice:?}");
    println!("  bob   = {bob:?}");

    section("Copy 与 Drop 互斥");
    // 实现了 Drop 的类型不能再 Copy（反之亦然）。
    // 想象一下：如果 String 既是 Copy 又有 Drop，复制后再析构两次 → 双重释放。
    println!("  （有析构需求的类型必然不能 Copy）");

    section("数组的 Copy 行为取决于元素");
    let arr1 = [1, 2, 3]; // [i32; 3] 是 Copy
    let arr2 = arr1;
    println!("  arr1 = {arr1:?}, arr2 = {arr2:?}（i32 数组是 Copy）");
    let vec1 = vec![1, 2, 3]; // Vec 不是 Copy
    let _vec2 = vec1.clone();
    // 下面会失败：let vec2 = vec1;  // Vec 不是 Copy
    println!("  Vec 必须 clone()（不能直接复制）");
}
