//! 16.3 HashMap<K, V> —— 哈希表
//!
//! 关键结论：
//! - HashMap 用哈希函数 + 数组实现，平均 O(1) 查/插/删。
//! - key 必须 `Hash + Eq`；自定义类型 derive 这两个即可。
//! - entry API：`entry(k).or_insert(v)` —— 不存在才插入，返回 &mut（原子操作）。
//! - `get`/`get_mut` 借用查询；`Borrow` trait 让 &str 能查 String 键。
//! - 无序：迭代顺序不确定（每次可能不同）。
//! - 想要有序 → 用 BTreeMap。
//!
//! 运行：`cargo run -p ch16_collections --example 03_hashmap`

use ch16_collections::{Point, char_frequency, section};
use std::collections::HashMap;

fn main() {
    section("创建与基本操作");
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("alice", 10);
    scores.insert("bob", 20);
    println!("  {scores:?}");

    section("get：查询（返回 Option<&V>）");
    if let Some(s) = scores.get("alice") {
        println!("  alice 的分数 = {s}");
    }
    println!("  carol 的分数 = {:?}（不存在）", scores.get("carol"));

    section("insert 覆盖旧值");
    let old = scores.insert("alice", 100); // 返回旧的 Some(10)
    println!("  覆盖 alice: 旧值 = {old:?}, 现在 = {:?}", scores.get("alice"));

    section("entry API：不存在才插入（原子）");
    // 统计词频的经典模式：entry().or_insert(0)。
    let text = "hello world hello rust world world";
    let mut freq: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    println!("  词频: {freq:?}");

    section("entry 的其它形态：or_insert_with");
    let mut map: HashMap<&str, Vec<i32>> = HashMap::new();
    // 不存在时用闭包构造默认值（惰性，比 or_insert(Vec::new()) 更省）。
    map.entry("even").or_default().push(2);
    map.entry("even").or_default().push(4);
    map.entry("odd").or_default().push(1);
    println!("  {map:?}");

    section("remove：删除并返回旧值");
    let removed = scores.remove("bob");
    println!("  remove(bob) = {removed:?}, 剩余 keys: {:?}", scores.keys().collect::<Vec<_>>());

    section("迭代 HashMap（无序）");
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    println!("  迭代（顺序不确定）:");
    for (k, v) in &map {
        println!("    {k} => {v}");
    }

    section("自定义类型作 key（需 Hash + Eq）");
    let mut grid: HashMap<Point, &str> = HashMap::new();
    grid.insert(Point::new(0, 0), "origin");
    grid.insert(Point::new(1, 2), "marker");
    grid.insert(Point::new(1, 2), "dup"); // 覆盖（Point 实现了 Eq）
    println!("  grid[(0,0)] = {:?}", grid.get(&Point::new(0, 0)));
    println!("  grid[(1,2)] = {:?}（被覆盖）", grid.get(&Point::new(1, 2)));

    section("Borrow：用 &str 查 String 键（无需构造 String）");
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("alice".to_string(), 30);
    // 直接传 &str 查 String 键，不分配。
    let age = map.get("alice");
    println!("  get(\"alice\") on HashMap<String,_> = {age:?}");

    section("用迭代器/collect 批量构造");
    let pairs = vec![("x", 1), ("y", 2), ("z", 3)];
    let map: HashMap<&str, i32> = pairs.into_iter().collect();
    println!("  from pairs: {map:?}");

    section("char_frequency：库提供的字符频率工具");
    let freq = char_frequency("abracadabra");
    println!("  'abracadabra' 字符频率: {freq:?}");
    // 找出现最多的字符。
    if let Some((&ch, &count)) = freq.iter().max_by_key(|&(_, c)| c) {
        println!("  最多: '{ch}' 出现 {count} 次");
    }
}
