//! 15.2 迭代器适配器（adapters）：变换迭代器
//!
//! 关键结论：
//! - 适配器「接收迭代器，返回新迭代器」，本身不消费（惰性）。
//! - 常用适配器：
//!   * `map(f)`        —— 对每个元素应用 f
//!   * `filter(p)`     —— 只保留满足谓词 p 的元素
//!   * `filter_map(f)` —— map + filter 二合一（f 返回 Option）
//!   * `take(n)`       —— 只取前 n 个
//!   * `skip(n)`       —— 跳过前 n 个
//!   * `zip(other)`    —— 配对两个迭代器
//!   * `enumerate()`   —— 加索引 (0, x), (1, x)...
//!   * `flat_map(f)`   —— map 后再「展平」一层
//!   * `chain(other)`  —— 串联两个迭代器
//!   * `rev()`         —— 反向（需 DoubleEndedIterator）
//!   * `step_by(n)`    —— 每隔 n-1 个取一个
//!
//! 运行：`cargo run -p ch15_iterators --example 02_adapters`

use ch15_iterators::section;

fn main() {
    section("map：变换每个元素");
    let nums = [1, 2, 3, 4];
    let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();
    println!("  map(*2): {doubled:?}");

    section("filter：保留满足条件的");
    let evens: Vec<&i32> = nums.iter().filter(|&&x| x % 2 == 0).collect();
    println!("  filter(偶数): {evens:?}");

    section("filter_map：map + filter 二合一");
    // 把字符串解析成数字，跳过失败的。
    let inputs = ["1", "2", "abc", "4", "xyz"];
    let parsed: Vec<i32> = inputs
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("  filter_map(parse): {parsed:?}");

    section("take / skip");
    let first2: Vec<&i32> = nums.iter().take(2).collect();
    let after2: Vec<&i32> = nums.iter().skip(2).collect();
    println!("  take(2): {first2:?}");
    println!("  skip(2): {after2:?}");

    section("zip：配对两个迭代器");
    let names = ["alice", "bob", "carol"];
    let ages = [30, 25, 41];
    let pairs: Vec<(&&str, &i32)> = names.iter().zip(ages.iter()).collect();
    println!("  zip: {pairs:?}");

    section("enumerate：加索引");
    for (i, name) in names.iter().enumerate() {
        println!("  #{i}: {name}");
    }

    section("flat_map：map 后展平");
    let words = ["hello world", "foo bar"];
    // split 后是嵌套迭代器，flat_map 把它们展平成一个。
    let tokens: Vec<&str> = words.iter().flat_map(|s| s.split_whitespace()).collect();
    println!("  flat_map(split): {tokens:?}");

    section("chain：串联两个迭代器");
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let chained: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("  chain: {chained:?}");

    section("rev：反向（需 DoubleEndedIterator）");
    let reversed: Vec<&i32> = nums.iter().rev().collect();
    println!("  rev: {reversed:?}");

    section("step_by：按步长取");
    let stepped: Vec<i32> = (0..10).step_by(3).collect();
    println!("  (0..10).step_by(3): {stepped:?}");

    section("windows / chunks（切片方法，不是迭代器但相关）");
    let arr = [1, 2, 3, 4, 5];
    let windows: Vec<&[i32]> = arr.windows(2).collect();
    println!("  windows(2): {windows:?}");
    let chunks: Vec<&[i32]> = arr.chunks(2).collect();
    println!("  chunks(2): {chunks:?}");
}
