//! 15.5 迭代器链实战 + 惰性求值 + collect 类型推断
//!
//! 关键结论：
//! - 迭代器链可以「任意组合」适配器，最后用一个消费者收尾。
//! - 惰性：map/filter/filter_map 等不立即执行，直到 collect/sum 等消费它们。
//! - collect 的目标类型由「左侧的类型标注」决定 —— 同一链可收集成不同集合。
//! - 编译器把整条链优化成紧凑循环 —— 接近手写代码的性能。
//!
//! 运行：`cargo run -p ch15_iterators --example 05_chains`

use ch15_iterators::section;
use std::collections::HashMap;

fn main() {
    section("实战 1：过滤 + 变换 + 收集");
    // 取 1..=20 中的偶数，平方，收集成 Vec。
    let result: Vec<i32> = (1..=20)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("  偶数平方: {result:?}");

    section("实战 2：从字符串解析数字并求和");
    let text = "10 20 abc 30 40 xyz 50";
    let total: i32 = text
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .sum();
    println!("  可解析的数字之和 = {total}");

    section("实战 3：统计词频（collect 成 HashMap）");
    let text = "the quick brown fox the lazy dog the end";
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    // 按 value 排序输出。
    let mut entries: Vec<_> = freq.into_iter().collect();
    entries.sort_by_key(|&(_, c)| std::cmp::Reverse(c));
    println!("  词频排序:");
    for (word, count) in &entries {
        println!("    {word}: {count}");
    }

    section("实战 4：enumerate + 过滤索引");
    let names = ["alice", "bob", "", "carol", ""];
    // 过滤掉空字符串，保留「原始索引 + 名字」。
    let indexed: Vec<(usize, &&str)> = names
        .iter()
        .enumerate()
        .filter(|&(_, s)| !s.is_empty())
        .collect();
    println!("  非空及其原索引: {indexed:?}");

    section("实战 5：flat_map 处理嵌套");
    let sentences = ["hello world", "rust is great", "learn by doing"];
    // 统计所有单词的「总字符数」。
    let total_chars: usize = sentences
        .iter()
        .flat_map(|s| s.split_whitespace())
        .map(|w| w.len())
        .sum();
    println!("  所有单词字符数之和 = {total_chars}");

    section("实战 6：zip + map 做点积");
    let v1 = [1, 2, 3];
    let v2 = [4, 5, 6];
    let dot: i32 = v1.iter().zip(v2.iter()).map(|(&a, &b)| a * b).sum();
    println!("  [1,2,3]·[4,5,6] = {dot}");

    section("实战 7：用 fold 构建复杂结果");
    // 把 [1,2,3,4] 分成 (奇数之和, 偶数之和)。
    let nums = [1, 2, 3, 4, 5, 6];
    let (odd_sum, even_sum) = nums.iter().fold((0, 0), |(odd, even), &x| {
        if x % 2 == 0 {
            (odd, even + x)
        } else {
            (odd + x, even)
        }
    });
    println!("  奇数和={odd_sum}, 偶数和={even_sum}");

    section("collect 类型推断：同一链不同目标");
    let nums = [1, 2, 3, 2, 1];
    // 教学：演示 collect 的类型推断（拷贝用 to_vec 更直接）。
    #[allow(clippy::iter_cloned_collect)]
    let as_vec: Vec<i32> = nums.iter().copied().collect();
    let as_set: std::collections::HashSet<i32> = nums.iter().copied().collect();
    let as_string: String = nums.iter().map(|n| char::from_digit(*n as u32, 10).unwrap()).collect();
    println!("  Vec: {as_vec:?}");
    println!("  HashSet: {as_set:?}");
    println!("  String: {as_string:?}");

    section("scan：带状态的 map（保留累加器）");
    // 计算前缀和：1, 1+2, 1+2+3...
    let prefix_sums: Vec<i32> = [1, 2, 3, 4]
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    println!("  前缀和: {prefix_sums:?}");
}
