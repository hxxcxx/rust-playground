//! 15.6 字符串 / Range / HashMap 迭代 + 性能对比
//!
//! 关键结论：
//! - 很多类型都「可迭代」：String（chars/bytes/lines）、Range、HashMap（key/value）。
//! - 迭代器 vs 手写 for 循环：性能几乎相同（迭代器被优化成等价循环）。
//! - 借用迭代（.iter()）vs 消费迭代（.into_iter()）的选择。
//!
//! 运行：`cargo run -p ch15_iterators --example 06_strings_ranges`

use ch15_iterators::section;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    section("字符串迭代：chars / bytes / lines");
    let text = "Hello, 世界!";
    // chars() 按 Unicode 标量值迭代（不是字节！）。
    let char_count = text.chars().count();
    let chars: Vec<char> = text.chars().collect();
    println!("  chars: {chars:?} (共 {char_count} 个)");
    // bytes() 按字节迭代（教学：演示字节迭代；实际计数用 .len() 更快）。
    #[allow(clippy::bytes_count_to_len)]
    let byte_count = text.bytes().count();
    println!("  bytes: {byte_count}");
    // lines() 按行迭代。
    let multi = "line1\nline2\nline3";
    let lines: Vec<&str> = multi.lines().collect();
    println!("  lines: {lines:?}");

    section("Range 迭代器：丰富的适配");
    // 0..n 是 Range，本身就是迭代器。
    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
    println!("  (1..=5).map(x*x): {squares:?}");
    // 包含与不包含。
    println!("  0..5   有 {} 个", (0..5).count());
    println!("  0..=5  有 {} 个", (0..=5).count());

    section("HashMap 迭代：key / value / entry");
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    // 直接迭代得到 (&K, &V)。
    for (k, v) in &map {
        println!("  {k} => {v}");
    }
    let keys: Vec<&&str> = map.keys().collect();
    let _ = keys; // keys 是对 &str 的引用（HashMap<&str,_> 的 key 类型）
    let values: Vec<&i32> = map.values().collect();
    println!("  values: {values:?}");

    section("BTreeMap 迭代是「有序」的");
    let mut btree = std::collections::BTreeMap::new();
    btree.insert("banana", 2);
    btree.insert("apple", 5);
    btree.insert("cherry", 8);
    // BTreeMap 按 key 排序迭代。
    for (k, v) in &btree {
        println!("  {k}: {v}");
    }

    section("性能对比：迭代器链 vs 手写循环");
    let n = 1_000_000;
    let nums: Vec<i32> = (0..n).collect();

    // 手写循环
    let t = Instant::now();
    let mut sum_manual = 0i64;
    for &x in &nums {
        if x % 2 == 0 {
            sum_manual += (x as i64) * 2;
        }
    }
    let manual_time = t.elapsed();

    // 迭代器链
    let t = Instant::now();
    let sum_iter: i64 = nums
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| (x as i64) * 2)
        .sum();
    let iter_time = t.elapsed();

    println!("  手写循环: sum={sum_manual}, 耗时 {manual_time:?}");
    println!("  迭代器链: sum={sum_iter}, 耗时 {iter_time:?}");
    println!("  （结果相同，性能接近 —— 迭代器被优化成等价循环）");

    section("collect 的 FromIterator：自定义类型也可收集");
    // 任何实现 FromIterator 的类型都能作为 collect 目标。
    // 例如 String 实现了 FromIterator<char>。
    let only_vowels: String = "Hello World".chars().filter(|c| "aeiouAEIOU".contains(*c)).collect();
    println!("  元音字母: {only_vowels:?}");
}
