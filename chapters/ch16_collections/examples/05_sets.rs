//! 16.5 HashSet / BTreeSet —— 集合
//!
//! 关键结论：
//! - Set 是「只有 key、没有 value」的 Map。
//! - HashSet：O(1) 平均，无序，元素需 Hash + Eq。
//! - BTreeSet：O(log n)，有序，元素需 Ord。
//! - 核心操作：插入、判断包含、去重。
//! - 集合运算：并集 union / 交集 intersection / 差集 difference / 对称差 symmetric_difference。
//! - 经典用途：去重、成员判断、标签系统。
//!
//! 运行：`cargo run -p ch16_collections --example 05_sets`

use ch16_collections::section;

fn main() {
    section("HashSet 基本：去重 + 成员判断");
    let mut fruits: std::collections::HashSet<&str> = std::collections::HashSet::new();
    fruits.insert("apple");
    fruits.insert("banana");
    fruits.insert("apple"); // 重复，无效果
    fruits.insert("cherry");
    println!("  fruits = {fruits:?}（apple 只有一份）");
    println!("  contains apple? {}", fruits.contains("apple"));
    println!("  contains grape? {}", fruits.contains("grape"));

    section("用 HashSet 给 Vec 去重");
    let nums = vec![1, 2, 2, 3, 3, 3, 4];
    let unique: std::collections::HashSet<i32> = nums.into_iter().collect();
    println!("  去重后: {unique:?}");

    section("BTreeSet：有序集合");
    let mut set: std::collections::BTreeSet<i32> = std::collections::BTreeSet::new();
    set.insert(3);
    set.insert(1);
    set.insert(4);
    set.insert(1);
    set.insert(5);
    // 迭代是升序的。
    println!("  BTreeSet（升序）: {set:?}");

    section("集合运算：并集 union");
    let a: std::collections::HashSet<i32> = [1, 2, 3].into_iter().collect();
    let b: std::collections::HashSet<i32> = [2, 3, 4].into_iter().collect();
    let union: std::collections::HashSet<i32> = a.union(&b).copied().collect();
    println!("  {{1,2,3}} ∪ {{2,3,4}} = {union:?}");

    section("交集 intersection");
    let inter: std::collections::HashSet<i32> = a.intersection(&b).copied().collect();
    println!("  {{1,2,3}} ∩ {{2,3,4}} = {inter:?}");

    section("差集 difference（a 有、b 没有）");
    let diff: std::collections::HashSet<i32> = a.difference(&b).copied().collect();
    println!("  {{1,2,3}} - {{2,3,4}} = {diff:?}");

    section("对称差 symmetric_difference（只在一方）");
    let sym: std::collections::HashSet<i32> = a.symmetric_difference(&b).copied().collect();
    println!("  对称差 = {sym:?}");

    section("子集 / 超集判断");
    let big: std::collections::HashSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    let small: std::collections::HashSet<i32> = [2, 3].into_iter().collect();
    println!("  {{2,3}} ⊆ {{1..5}} ? {}", small.is_subset(&big));
    println!("  {{1..5}} ⊇ {{2,3}} ? {}", big.is_superset(&small));
    println!("  {{2,3}} 与 {{4,5}} 不相交 ? {}", small.is_disjoint(&[4, 5].into_iter().collect::<std::collections::HashSet<i32>>()));

    section("实战：标签系统（用户兴趣交集）");
    let alice: std::collections::HashSet<&str> = ["rust", "linux", "systems"].into_iter().collect();
    let bob: std::collections::HashSet<&str> = ["rust", "web", "typescript"].into_iter().collect();
    let common: Vec<&&str> = alice.intersection(&bob).collect();
    println!("  共同兴趣: {common:?}");

    section("remove / take：删除元素");
    let mut set: std::collections::HashSet<i32> = [1, 2, 3].into_iter().collect();
    let removed = set.remove(&2);
    println!("  remove(2) 返回是否曾存在: {removed}, set={set:?}");
    let taken = set.take(&1); // take 返回被移除的值
    println!("  take(1) = {taken:?}, set={set:?}");
}
