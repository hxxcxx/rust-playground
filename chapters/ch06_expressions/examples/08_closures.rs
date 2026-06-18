//! 6.8 闭包：轻量级函数值
//!
//! 关键结论：
//! - 闭包语法：`|参数| 表达式` 或 `|参数| { ... }`，类型通常可推断。
//! - 调用语法与函数相同：`closure(args)`。
//! - 闭包能「捕获」外层作用域的变量（按引用 / 按值移动 / 按可变引用）。
//! - 没有显式返回类型时，闭包主体可以是单个表达式；指定返回类型时必须用 `{}`。
//! - 详细见第 14 章，这里只介绍语法。
//!
//! 运行：`cargo run -p ch06_expressions --example 08_closures`

use ch06_expressions::section;

fn main() {
    section("最简单的闭包：推断类型");
    #[allow(clippy::manual_is_multiple_of)]
    let is_even = |x| x % 2 == 0;
    println!("  is_even(14) = {}", is_even(14));
    println!("  is_even(15) = {}", is_even(15));

    section("显式标注类型 + 返回类型（必须用 {}）");
    #[allow(clippy::manual_is_multiple_of)]
    let is_even = |x: u64| -> bool { x % 2 == 0 };
    println!("  is_even(14u64) = {}", is_even(14));

    section("捕获外层变量（按引用）");
    let factor = 10;
    let multiply = |x| x * factor; // factor 被借用
    println!("  multiply(5) = {}", multiply(5));
    println!("  factor 仍可用: {factor}");

    section("捕获可变引用（修改外层变量）");
    let mut total = 0;
    let mut add_to_total = |n| total += n; // total 被可变借用
    add_to_total(10);
    add_to_total(20);
    println!("  total = {total}");

    section("移动捕获（按值）—— 使用 move 关键字");
    let name = String::from("Rust");
    let greet = move || println!("  hello, {name}!"); // name 被移动进闭包
    greet();
    // println!("{name}"); // ❌ name 已被移动

    section("闭包作为函数参数（典型用法：排序比较函数）");
    let mut nums = vec![5, 2, 8, 1, 9, 3];
    nums.sort(); // 升序（自然顺序）
    println!("  升序: {nums:?}");
    nums.sort_by(|a, b| b.cmp(a)); // 降序：自定义比较函数
    println!("  降序: {nums:?}");

    section("闭包作为迭代器参数（最常见）");
    let nums = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();
    let evens: Vec<&i32> = nums.iter().filter(|&&x| x % 2 == 0).collect();
    println!("  原始: {nums:?}");
    println!("  翻倍: {doubled:?}");
    println!("  偶数: {evens:?}");
}
