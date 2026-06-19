//! 16.1 Vec<T> —— 动态数组
//!
//! 关键结论：
//! - Vec 是「连续内存 + 长度 + 容量」的三元组；末尾 push/pop 是 O(1) 均摊。
//! - 容量（capacity）：预分配的空间；长度（len）：实际元素数。
//! - 超容量时按 ~2 倍扩容 + 搬迁 —— 所以批量 push 前 `with_capacity` 能避免多次扩容。
//! - 随机访问 O(1)（下标）；中间插入/删除 O(n)（要搬移后面元素）。
//! - 常用方法：push/pop/insert/remove/extend/retain/drain/split_off/resize。
//!
//! 运行：`cargo run -p ch16_collections --example 01_vec`

use ch16_collections::section;

fn main() {
    section("创建 Vec");
    let v1: Vec<i32> = Vec::new(); // 空
    let v2 = vec![1, 2, 3]; // 宏创建
    let v3: Vec<i32> = (0..5).collect(); // 从迭代器
    println!("  Vec::new() = {v1:?}");
    println!("  vec![1,2,3] = {v2:?}");
    println!("  (0..5).collect() = {v3:?}");

    section("push / pop：末尾 O(1)");
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    println!("  push 后: {v:?}");
    let last = v.pop();
    println!("  pop() = {last:?}, v = {v:?}");

    section("容量 vs 长度");
    let mut v: Vec<i32> = Vec::with_capacity(10);
    println!("  with_capacity(10): len={}, cap={}", v.len(), v.capacity());
    for i in 0..5 {
        v.push(i);
    }
    println!("  push 5 个后: len={}, cap={}（容量未变）", v.len(), v.capacity());
    v.push(100);
    // 不超过容量，cap 仍是 10。
    println!("  再 push 1 个: len={}, cap={}", v.len(), v.capacity());

    section("扩容行为观察");
    let mut v: Vec<i32> = Vec::new();
    let mut prev_cap = 0;
    for i in 0..20 {
        v.push(i);
        if v.capacity() != prev_cap {
            println!("    len={} 时扩容: cap {} → {}", v.len(), prev_cap, v.capacity());
            prev_cap = v.capacity();
        }
    }

    section("随机访问 O(1)，越界 panic");
    let v = [10, 20, 30];
    println!("  v[1] = {}", v[1]);
    println!("  v.get(5) = {:?}（安全，不 panic）", v.get(5));

    section("insert / remove：中间 O(n)");
    let mut v = vec![1, 2, 4, 5];
    v.insert(2, 3); // 在索引 2 插入
    println!("  insert(2,3) 后: {v:?}");
    v.remove(0); // 删除索引 0
    println!("  remove(0) 后: {v:?}");

    section("retain / drain / split_off：批量操作");
    let mut v = vec![1, 2, 3, 4, 5, 6];
    v.retain(|&x| x % 2 == 0); // 只保留偶数
    println!("  retain(偶数): {v:?}");

    let mut v = vec![1, 2, 3, 4, 5];
    let drained: Vec<i32> = v.drain(1..3).collect(); // 抽出 [1,3)
    println!("  drain(1..3) 抽出: {drained:?}, 剩余: {v:?}");

    let mut v = vec![1, 2, 3, 4, 5];
    let tail = v.split_off(3); // 从索引 3 处一分为二
    println!("  split_off(3): 前={v:?}, 后={tail:?}");

    section("resize / extend / truncate");
    let mut v = vec![1, 2];
    v.resize(5, 0); // 扩到 5，新位置填 0
    println!("  resize(5,0): {v:?}");
    v.truncate(3); // 截到 3
    println!("  truncate(3): {v:?}");
    v.extend([10, 20]);
    println!("  extend([10,20]): {v:?}");

    section("排序 / 二分查找（Vec 需先排序）");
    let mut v = vec![5, 1, 4, 2, 3];
    v.sort();
    println!("  sort 后: {v:?}");
    if let Ok(idx) = v.binary_search(&3) {
        println!("  binary_search(3) → 索引 {idx}");
    }
}
