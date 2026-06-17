//! 5.2 使用引用
//!
//! 关键结论：
//! - Rust 引用与 C++ 引用不同：用 `&` 显式创建、`*` 显式解引用；
//!   但 `.` 运算符会「隐式」解引用左操作数，并在方法调用时「隐式」借用左操作数。
//! - 给引用变量赋值会让它「指向新位置」（C++ 是写入被引用对象）。
//! - 引用可以套引用（`&&T`），`.` 和比较运算符会穿透多层引用。
//! - `==` 比较的是「最终指向的值」；要比较地址用 `std::ptr::eq`。
//! - 引用「永远不为空」；需要可空时用 `Option<&T>`（运行时与 C 的 NULL 一样高效）。
//! - 可以借用任意表达式的引用：Rust 创建匿名变量保存中间结果。
//! - 「胖指针」：切片引用 `&[T]` / `&str`（地址+长度），特征对象引用（地址+vtable）。
//!
//! 运行：`cargo run -p ch05_references --example 02_using_references`
//
// 教学示例：刻意保留 `(*r).x`、`(&mut w).sort()` 等显式写法以演示「与隐式写法等价」。
#![allow(clippy::explicit_auto_deref, clippy::needless_borrow, clippy::op_ref)]

use ch05_references::{Point, section};

/// 演示：Rust 用 `&`/`*` 显式创建和解引用，与 C++ 的隐式行为不同。
fn explicit_operators() {
    let x = 10;
    let r = &x; // &x 是共享引用，类型 &i32
    assert_eq!(*r, 10); // 必须用 * 显式解引用

    let mut y = 32;
    let m = &mut y; // &mut y 是可变引用，类型 &mut i32
    *m += 32; // 通过 *m 修改 y
    assert_eq!(*m, 64);
    println!("显式：r = {r}（*r 解引用）, y = {y}（通过 *m 修改为 64）");
}

/// 演示：`.` 运算符会「隐式」解引用左操作数。
/// 在方法调用时，`.` 还会「隐式」借用左操作数的引用。
fn dot_operator_auto() {
    let aria = Point { x: 1, y: 729 };
    let anime_ref = &aria;
    // 隐式解引用：anime_ref.name 等价于 (*anime_ref).name
    assert_eq!(anime_ref.x, 1);
    assert_eq!((*anime_ref).x, 1); // 等价的显式写法

    // 方法调用时隐式借用：v.sort() 等价于 (&mut v).sort()
    let mut v = vec![1973, 1968];
    v.sort(); // 隐式借用 &mut v
    let mut w = vec![3, 1, 2];
    (&mut w).sort(); // 等价的显式写法
    println!("隐式：v = {v:?}, w = {w:?}");
}

/// 演示：给引用变量赋值，是让它「指向新位置」（而不是写入被引用对象）。
/// 这是 Rust 与 C++ 引用最显著的行为差异之一。
fn reassign_reference() {
    let x = 10;
    let y = 20;
    let mut r = &x; // r 初始指向 x
    let b = true;
    if b {
        r = &y; // 让 r 指向 y（不是把 y 的值写入 x！）
    }
    assert!(*r == 10 || *r == 20);
    println!("引用赋值：r 现在指向 y，*r = {r}");
}

/// 演示：引用的引用 —— `.` 运算符会穿过所有引用层。
fn ref_of_ref() {
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;

    // rrr.y 要穿越三层引用才能找到 Point，再取 y
    assert_eq!(rrr.y, 729);
    println!("引用链：&&&Point 的 .y = {}（穿越 3 层）", rrr.y);
}

/// 演示：比较运算符也会穿透引用。
/// `==` 比较的是「最终指向的值」；要比较「是否同一地址」用 `std::ptr::eq`。
fn compare_references() {
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;

    assert!(rrx == rry); // == 穿越 2 层引用比较 x 和 y 的值，相等
    assert!(rx == ry); // 同上
    assert!(!std::ptr::eq(rx, ry)); // 但 rx 和 ry 指向不同地址
    println!("== 穿透引用比较值：rx == ry 为 true");
    println!(
        "std::ptr::eq 比较地址：rx 与 ry 是 {} 同一地址",
        if std::ptr::eq(rx, ry) { "" } else { "非" }
    );
}

/// 演示：引用「永远不为空」；需要可空引用时用 `Option<&T>`。
/// 机器层面 `None` 编码为全 0（即 null 指针），`Some(r)` 为非零地址 —— 与 C 的 NULL 一样高效。
fn never_null() {
    let x = 42;
    // 引用本身绝不可能是 null：
    let r: &i32 = &x;

    // 需要表示「可能没有引用」时用 Option<&T>：
    let maybe: Option<&i32> = None;
    let some: Option<&i32> = Some(r);
    println!("引用本身不为空：*r = {r}");
    println!("Option<&T> 可以是 None = {maybe:?} 或 Some = {some:?}");
    println!(
        "Option<&T> 的大小 = {} 字节（与裸指针一样）",
        std::mem::size_of::<Option<&i32>>()
    );
}

/// 演示：可以借用「任意表达式」的引用。
/// Rust 会创建匿名变量保存表达式结果，再让引用指向它。
fn borrow_any_expression() {
    // factorial(6) 的结果被存入一个匿名变量，r 指向它
    let r = &factorial(6);
    // 算术运算符可穿透一层引用
    assert_eq!(*r + &1009, 720 + 1009);
    println!("&factorial(6) = {r}, *r + &1009 = {}", *r + &1009);
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

/// 演示：「胖指针」—— 切片引用与特征对象引用。
/// 切片引用 `&[T]` / `&str` 携带「地址 + 长度」两个字。
fn fat_pointers() {
    let v = [10, 20, 30, 40, 50];
    let s: &[i32] = &v[1..4]; // 切片引用：胖指针，携带起始地址和长度 3
    assert_eq!(s, [20, 30, 40]);
    println!("切片引用 &v[1..4] = {s:?}（携带地址 + 长度）");

    let name: &str = "shirataki"; // &str 也是胖指针：地址 + 字节长度
    println!("&str = {name:?}（同样是胖指针）");

    // 普通引用是 1 个字，切片引用是 2 个字
    println!(
        "size: &i32 = {} 字节, &[i32] = {} 字节, &str = {} 字节",
        std::mem::size_of::<&i32>(),
        std::mem::size_of::<&[i32]>(),
        std::mem::size_of::<&str>()
    );
}

fn main() {
    section("显式 `&` / `*`：与 C++ 引用的关键区别");
    explicit_operators();

    section("`.` 运算符的隐式解引用与隐式借用");
    dot_operator_auto();

    section("给引用赋值 = 指向新位置（非写入被引用对象）");
    reassign_reference();

    section("引用的引用：`.` 穿透多层引用");
    ref_of_ref();

    section("比较引用：`==` 穿透 vs `std::ptr::eq` 比地址");
    compare_references();

    section("引用永不为空：用 Option<&T> 表示可空");
    never_null();

    section("借用任意表达式的引用：匿名变量");
    borrow_any_expression();

    section("胖指针：切片引用 / &str / 特征对象引用");
    fat_pointers();
}
