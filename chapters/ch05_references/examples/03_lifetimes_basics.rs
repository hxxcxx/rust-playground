//! 5.3.1 引用安全 —— 借用局部变量（生命周期基础）
//!
//! 关键结论：
//! - 「生命周期」（lifetime）是编译期虚构概念：一段引用可以安全使用的时间段。
//! - 两条核心约束：
//!   ① 对 `x` 的引用的生命周期必须「被包围在」`x` 自己的生命周期内（不能比 x 长）；
//!   ② 存入变量 `r` 的引用，其生命周期必须「包围」`r` 的整个使用期（不能比 r 短）。
//! - 第一条限制最大范围，第二条限制最小范围；编译器要找到一个能同时满足的生命周期。
//! - 借用更大结构的「一部分」（如 `&v[1]`）规则相同：借用必须被所有者包围。
//!
//! 运行：`cargo run -p ch05_references --example 03_lifetimes_basics`

use ch05_references::section;

/// ✅ 正例：引用的生命周期被 `x` 完全包围，又能包围整个 `r` 的使用 —— 满足约束。
fn good_borrow() {
    let x = 1;
    let r = &x; // 引用生命周期：从此处到语句块结束，完全在 x 的生命期内
    assert_eq!(*r, 1);
    println!("✅ 借用合法：*r = {r}");
}

/// ✅ 正例：借用 Vec 的一个元素，规则与借用局部变量相同。
/// `v` 拥有 Vec，Vec 拥有元素；所以 `&v[1]` 必须被 `v` 的生命周期包围。
fn borrow_part_of_vec() {
    let v = [1, 2, 3]; // 数组同样能演示「借用其中元素」
    let r = &v[1]; // 借用 v 的一个元素
    assert_eq!(*r, 2);
    println!("✅ 借用数组元素：*r = {r}");
}

/// ❌ 反面教材（注释保留）：把对内层 `x` 的引用带出 `x` 的作用域 —— 悬空引用。
/// Rust 拒绝编译，错误信息：
///   error: `x` does not live long enough
///   borrowed value does not live long enough / `x` dropped here while still borrowed
fn bad_borrow_commented() {
    println!("\n❌ 反面教材（见源码注释）：");
    println!("{{");
    println!("    let r;");
    println!("    {{");
    println!("        let x = 1;");
    println!("        r = &x;   // ❌ 引用 x 但 x 即将离开作用域");
    println!("    }}            // x 在此被 drop，但 r 仍指向它");
    println!("    assert_eq!(*r, 1); // 悬空指针！");
    println!("}}");
    // 解开下面注释会出现 E0597: `x` does not live long enough
    // {
    //     let r;
    //     {
    //         let x = 1;
    //         r = &x;
    //     }
    //     assert_eq!(*r, 1);
    // }
}

/// 演示：把外层和内层顺序颠倒后，约束可满足。
/// 把 `s` 放到 `parabola` 仍在作用域的位置即可。
fn fixed_borrow() {
    let parabola = [9, 4, 1, 0, 1, 4, 9];
    let s = smallest(&parabola); // s 在 parabola 的生命期内使用
    assert_eq!(*s, 0);
    println!("✅ 修正后：在 parabola 作用域内使用 s，*s = {s}");
}

/// 返回切片中最小元素的引用。完整签名是 `fn smallest<'a>(v: &'a [i32]) -> &'a i32`。
/// （生命周期省略规则见 04_lifetimes_in_fn）
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

fn main() {
    section("正例：合法的局部借用");
    good_borrow();

    section("正例：借用 Vec 的一部分");
    borrow_part_of_vec();

    section("反例分析：悬空引用为何被拒绝");
    bad_borrow_commented();

    section("修正：让引用在所指对象生命期内使用");
    fixed_borrow();

    section("生命周期核心约束");
    println!("① 对 x 的引用生命周期 ≤ x 的生命周期（限制最大范围）");
    println!("② 存入 r 的引用生命周期 ≥ r 的使用期（限制最小范围）");
    println!("编译器要找到能同时满足两者的生命周期；找不到就报错。");
}
