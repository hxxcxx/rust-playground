//! 15.1 迭代器基础：Iterator trait / next / for / 三种 iter
//!
//! 关键结论：
//! - `Iterator` trait 只有一个必需方法：`fn next(&mut self) -> Option<Item>`。
//! - 返回 `Some(x)` 产出下一个元素；返回 `None` 表示结束。
//! - for 循环 = `IntoIterator::into_iter()` + 循环调用 next() 的语法糖。
//! - 三种从集合获取迭代器：
//!   * `iter()`    —— 借用 `&T`（只读，最常用）
//!   * `iter_mut()`—— 可变借用 `&mut T`（可修改元素）
//!   * `into_iter()`—— 消耗集合，产出 `T`（拿走所有权）
//!
//! 运行：`cargo run -p ch15_iterators --example 01_basics`

use ch15_iterators::section;

fn main() {
    section("手动调用 next()");
    let mut iter = [10, 20, 30].into_iter();
    println!("  next() = {:?}", iter.next());
    println!("  next() = {:?}", iter.next());
    println!("  next() = {:?}", iter.next());
    println!("  next() = {:?}", iter.next()); // None —— 结束

    section("for 循环 = 迭代器语法糖");
    for n in [1, 2, 3] {
        println!("  {n}");
    }

    section("Range 也是迭代器");
    for i in 0..3 {
        println!("  {i}");
    }
    let sum: i32 = (1..=10).sum();
    println!("  1..=10 求和 = {sum}");

    section("三种 iter 的区别");
    let v = vec![String::from("a"), String::from("b")];

    // 1) iter() —— 借用，产出 &String
    for s in v.iter() {
        println!("  iter(): {s} (借用)");
    }
    // v 还能用（只是借用）。
    println!("  v 还在: {v:?}");

    // 2) iter_mut() —— 可变借用，产出 &mut String
    let mut v = vec![1, 2, 3];
    for n in v.iter_mut() {
        *n *= 10;
    }
    println!("  iter_mut() 后: {v:?}");

    // 3) into_iter() —— 消耗，产出 String（所有权转移）
    let v = vec![String::from("x"), String::from("y")];
    for s in v.into_iter() {
        println!("  into_iter(): {s}（拿走所有权）");
    }
    // v 已被消耗，不能再访问。

    section("空迭代器：next() 立即返回 None");
    let empty: std::slice::Iter<'_, i32> = [].iter();
    println!("  空迭代器 next() = {:?}", empty.into_iter().next());

    section("迭代器是惰性的：不消费就不执行");
    let iter = [1, 2, 3].iter().map(|x| {
        println!("    处理 {x}");
        x * 2
    });
    println!("  （上面没有任何输出 —— map 还没被驱动）");
    // 消费它，map 才执行。
    let collected: Vec<_> = iter.collect();
    println!("  collect 后: {collected:?}");
}
