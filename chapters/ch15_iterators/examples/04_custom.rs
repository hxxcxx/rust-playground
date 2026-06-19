//! 15.4 自定义迭代器：实现 Iterator trait
//!
//! 关键结论：
//! - 只要实现 `fn next(&mut self) -> Option<Item>`，就免费获得所有适配器/消费者。
//! - 设计要点：用结构体保存「当前状态」（位置、下一个值等）。
//! - 关联类型 `Item` 决定产出什么类型。
//! - 想让类型支持 for 循环 → 实现 `IntoIterator`（或本身是 Iterator）。
//! - 进阶：`DoubleEndedIterator`（支持 rev）/ `ExactSizeIterator`（已知长度）。
//!
//! 运行：`cargo run -p ch15_iterators --example 04_custom`

use ch15_iterators::{Countdown, Evens, Tree, section};

fn main() {
    section("Countdown：手写 next() 的倒数迭代器");
    let cd = Countdown::new(3);
    for n in cd {
        println!("  {n}");
    }

    section("自定义迭代器免费获得所有适配器");
    // Countdown 实现了 Iterator，所以 map/filter/collect 全都能用！
    let squares: Vec<i32> = Countdown::new(5).map(|x| x * x).collect();
    println!("  Countdown(5).map(x*x): {squares:?}");
    let sum: i32 = Countdown::new(10).sum();
    println!("  sum(Countdown(10)) = {sum}");

    section("Evens：自定义「只产出偶数」的迭代器");
    let evens: Vec<i32> = Evens::new(1, 10).collect();
    println!("  Evens(1,10): {evens:?}");
    let sum_evens: i32 = Evens::new(0, 100).sum();
    println!("  sum(Evens(0,100)) = {sum_evens}");

    section("把自定义迭代器消费成其它集合");
    let set: std::collections::BTreeSet<i32> = Evens::new(0, 10).collect();
    println!("  Evens → BTreeSet: {set:?}");

    section("树的中序遍历（用 Vec 模拟迭代器）");
    let tree = Tree::node(
        4,
        Tree::node(2, Tree::node(1, Tree::leaf(), Tree::leaf()), Tree::node(3, Tree::leaf(), Tree::leaf())),
        Tree::node(6, Tree::node(5, Tree::leaf(), Tree::leaf()), Tree::node(7, Tree::leaf(), Tree::leaf())),
    );
    let in_order: Vec<i32> = tree.in_order();
    println!("  中序遍历: {in_order:?}（应该是升序 1..=7）");

    section("实现 IntoIterator：让自定义类型支持 for 循环");
    let matrix = Matrix { rows: vec![vec![1, 2, 3], vec![4, 5, 6]] };
    // for 直接遍历「每一行」。
    for row in &matrix {
        println!("    行: {row:?}");
    }

    section("DoubleEndedIterator：支持 .rev()");
    // 标准库 Vec 的迭代器实现了 DoubleEndedIterator。
    let reversed: Vec<&i32> = [1, 2, 3].iter().rev().collect();
    println!("  [1,2,3].rev(): {reversed:?}");
    // Countdown 也实现了（next_back 从尾部取）—— 这里演示标准库 Range。
    let back: Vec<i32> = (1..=5).rev().collect();
    println!("  (1..=5).rev(): {back:?}");
}

/// 一个矩阵：演示 IntoIterator（按行遍历）。
struct Matrix {
    rows: Vec<Vec<i32>>,
}

impl<'a> IntoIterator for &'a Matrix {
    type Item = &'a Vec<i32>;
    type IntoIter = std::slice::Iter<'a, Vec<i32>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter()
    }
}
