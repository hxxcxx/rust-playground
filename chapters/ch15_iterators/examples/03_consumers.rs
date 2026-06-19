//! 15.3 迭代器消费者（consumers）：得到结果
//!
//! 关键结论：
//! - 消费者「消耗迭代器」，产生最终结果（集合/数字/布尔）。
//! - 常用消费者：
//!   * `collect()`      —— 收集成集合（Vec/HashMap/String...）
//!   * `sum()/product()`—— 求和/求积
//!   * `count()`        —— 数个数
//!   * `min()/max()`    —— 极值
//!   * `fold(init, f)`  —— 折叠（类似 reduce）
//!   * `reduce(f)`      —— 折叠（无初始值）
//!   * `any(p)/all(p)`  —— 是否存在/全部满足
//!   * `find(p)/position(p)` —— 查找
//!   * `for_each(f)`    —— 逐个处理（副作用）
//!   * `partition(p)`   —— 按条件分两组
//!
//! 运行：`cargo run -p ch15_iterators --example 03_consumers`

use ch15_iterators::section;
use std::collections::HashMap;

fn main() {
    let nums = [1, 2, 3, 4, 5];

    section("collect：收集成 Vec");
    // 教学：演示 iter().copied().collect()（实际拷贝切片用 to_vec() 更直接）。
    #[allow(clippy::iter_cloned_collect)]
    let v: Vec<i32> = nums.iter().copied().collect();
    println!("  {v:?}");

    section("collect 成不同集合（类型推断决定）");
    // 用 turbofish 指定目标类型。
    let set: std::collections::HashSet<i32> = nums.iter().copied().collect();
    println!("  HashSet: {set:?}");
    let buf: std::collections::VecDeque<i32> = nums.iter().copied().collect();
    println!("  VecDeque: {buf:?}");

    section("collect 成 String");
    let chars = ['h', 'i', '!'];
    let s: String = chars.iter().collect();
    println!("  chars → String: {s:?}");

    section("collect 成 HashMap（从 (k,v) 对）");
    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    let map: HashMap<&str, i32> = pairs.into_iter().collect();
    println!("  HashMap: {map:?}");

    section("sum / product / count");
    let sum: i32 = nums.iter().sum();
    let product: i32 = nums.iter().product();
    let count = nums.iter().count();
    println!("  sum={sum}, product={product}, count={count}");

    section("min / max");
    let min = nums.iter().min();
    let max = nums.iter().max();
    println!("  min={min:?}, max={max:?}");

    section("fold：带初始值的折叠");
    // fold(0, |acc, x| ...) —— 累加器风格。
    // 教学：演示 fold（这个例子恰好等价于 sum，实际求和用 sum()）。
    #[allow(clippy::unnecessary_fold)]
    let sum_squares: i32 = nums.iter().map(|&x| x * x).fold(0, |acc, x| acc + x);
    println!("  各元素平方和 = {sum_squares}");
    // 用 fold 实现 join。
    let joined = nums.iter().fold(String::new(), |mut acc, &x| {
        if !acc.is_empty() {
            acc.push(',');
        }
        acc.push_str(&x.to_string());
        acc
    });
    println!("  fold join: {joined}");

    section("reduce：无初始值的折叠（用第一个元素作初始）");
    let product_reduce = nums.iter().copied().reduce(|a, b| a * b);
    println!("  reduce(*): {product_reduce:?}");

    section("any / all");
    let has_even = nums.iter().any(|&x| x % 2 == 0);
    let all_positive = nums.iter().all(|&x| x > 0);
    println!("  any(偶数)={has_even}, all(正数)={all_positive}");

    section("find / position");
    let found = nums.iter().find(|&&x| x == 3);
    let pos = nums.iter().position(|&x| x == 3);
    println!("  find(==3)={found:?}, position={pos:?}");

    section("partition：按条件分成两组");
    let (evens, odds): (Vec<i32>, Vec<i32>) = nums.iter().copied().partition(|&x| x % 2 == 0);
    println!("  偶数: {evens:?}, 奇数: {odds:?}");

    section("for_each：逐个处理");
    (0..3).for_each(|i| println!("    第 {i} 个"));

    section("nth：取第 n 个（跳过前 n，消费 1 个）");
    let mut iter = nums.iter();
    let third = iter.nth(2); // 跳过 0,1，取第 2 个
    println!("  nth(2) = {third:?}, 之后 next = {:?}", iter.next());
}
