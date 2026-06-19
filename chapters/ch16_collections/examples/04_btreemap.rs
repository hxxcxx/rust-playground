//! 16.4 BTreeMap<K, V> —— 有序映射
//!
//! 关键结论：
//! - BTreeMap 用 B 树实现，按 key「排序」存储，O(log n) 查/插/删。
//! - key 必须 `Ord`（不需要 Hash）。
//! - 迭代是「有序」的（升序）—— 这是相对 HashMap 的最大优势。
//! - `range`：范围查询，O(log n + k)。
//! - 适用：需要按顺序遍历、范围查询、稳定迭代的场景（如时间线、排行榜）。
//!
//! 运行：`cargo run -p ch16_collections --example 04_btreemap`

use ch16_collections::{sample_timeline, section};

fn main() {
    section("BTreeMap 自动按 key 排序");
    let mut map = std::collections::BTreeMap::new();
    map.insert("banana", 2);
    map.insert("apple", 5);
    map.insert("cherry", 8);
    map.insert("date", 3);
    // 即使乱序插入，迭代也是升序。
    println!("  迭代（升序）:");
    for (k, v) in &map {
        println!("    {k}: {v}");
    }

    section("first / last：最小/最大 key");
    println!("  最小 key: {:?}", map.first_key_value());
    println!("  最大 key: {:?}", map.last_key_value());

    section("pop_first / pop_last：弹出端点");
    let mut map = std::collections::BTreeMap::from([(1, "a"), (2, "b"), (3, "c")]);
    println!("  pop_first() = {:?}", map.pop_first());
    println!("  pop_last()  = {:?}", map.pop_last());
    println!("  剩余: {map:?}");

    section("range：范围查询（核心优势）");
    let mut map = std::collections::BTreeMap::new();
    for i in 0..10 {
        map.insert(i, i * i);
    }
    // 查 [3, 7) 范围。
    let range: Vec<(&i32, &i32)> = map.range(3..7).collect();
    println!("  range(3..7): {range:?}");
    // 闭区间用 ..= 。
    let range_inclusive: Vec<(&i32, &i32)> = map.range(..=2).collect();
    println!("  range(..=2): {range_inclusive:?}");
    // 从某 key 到末尾。
    let tail: Vec<(&i32, &i32)> = map.range(7..).collect();
    println!("  range(7..): {tail:?}");

    section("实战：事件时间线（按时间戳排序）");
    let timeline = sample_timeline();
    // 即使插入顺序是 30,10,20，遍历仍是 10,20,30。
    println!("  时间线（按时间升序）:");
    for (time, event) in &timeline {
        println!("    t={time}: {} - {}", event.name, event.detail);
    }

    section("实战：排行榜（按分数取前 N）");
    let mut scores = std::collections::BTreeMap::new();
    scores.insert(85, "Alice");
    scores.insert(92, "Bob");
    scores.insert(78, "Carol");
    scores.insert(95, "Dave");
    scores.insert(88, "Eve");
    // 取分数最高的 3 个（从大到小）。
    let top3: Vec<(&i32, &&str)> = scores.iter().rev().take(3).collect();
    println!("  Top 3:");
    for (score, name) in &top3 {
        println!("    {name}: {score}");
    }

    section("split_off：按 key 分裂成两个");
    let mut map = std::collections::BTreeMap::from([(1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let tail = map.split_off(&3); // key >= 3 的进 tail
    println!("  split_off(3): 前={map:?}, 后={tail:?}");

    section("HashMap vs BTreeMap 选择");
    println!("  HashMap: O(1) 平均，无序，key 需 Hash+Eq");
    println!("  BTreeMap: O(log n)，有序，key 需 Ord");
    println!("  → 需要排序/范围查询 → BTreeMap；否则 → HashMap");
}
