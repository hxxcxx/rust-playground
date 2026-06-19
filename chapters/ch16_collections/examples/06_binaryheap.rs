//! 16.6 BinaryHeap<T> —— 二叉堆 / 优先队列
//!
//! 关键结论：
//! - BinaryHeap 默认是「最大堆」：peek 总能拿到最大值。
//! - push / pop 是 O(log n)；peek 是 O(1)。
//! - 元素需 `Ord`。
//! - 用途：优先队列、TopK、Dijkstra 最短路、任务调度。
//! - 想要「最小堆」：用 Reverse 包装，或 std::cmp::Reverse。
//! - 不保证整体有序，只保证「堆顶是极值」。
//!
//! 运行：`cargo run -p ch16_collections --example 06_binaryheap`

use ch16_collections::section;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    section("最大堆：peek 总是最大值");
    let mut heap = BinaryHeap::new();
    heap.push(5);
    heap.push(1);
    heap.push(8);
    heap.push(3);
    println!("  push 5,1,8,3 后 peek = {:?}", heap.peek()); // 8
    // pop 按从大到小顺序出来。
    let mut sorted = Vec::new();
    while let Some(v) = heap.pop() {
        sorted.push(v);
    }
    println!("  全部 pop（降序）: {sorted:?}");

    section("用 BinaryHeap 做堆排序（降序）");
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let mut heap: BinaryHeap<i32> = nums.into_iter().collect();
    let mut descending = Vec::new();
    while let Some(v) = heap.pop() {
        descending.push(v);
    }
    println!("  堆排序（降序）: {descending:?}");

    section("最小堆：用 Reverse 包装");
    // Reverse 倒转 Ord 比较 —— 最小值变成「最大」浮到堆顶。
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for n in [5, 1, 8, 3] {
        min_heap.push(Reverse(n));
    }
    let mut ascending = Vec::new();
    while let Some(Reverse(v)) = min_heap.pop() {
        ascending.push(v);
    }
    println!("  最小堆 pop（升序）: {ascending:?}");

    section("优先队列：按优先级处理任务");
    let mut pq: BinaryHeap<Task> = BinaryHeap::new();
    pq.push(Task { priority: 1, name: "低优先级" });
    pq.push(Task { priority: 10, name: "高优先级" });
    pq.push(Task { priority: 5, name: "中优先级" });
    println!("  按优先级处理:");
    while let Some(task) = pq.pop() {
        println!("    [优先级 {}] {}", task.priority, task.name);
    }

    section("TopK 问题：流式保留最大的 K 个");
    let stream = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let k = 3;
    // 用「最小堆」保留最大的 K 个：新元素比堆顶大就替换堆顶。
    let mut topk: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for &n in &stream {
        if topk.len() < k {
            topk.push(Reverse(n));
        } else if let Some(&Reverse(min)) = topk.peek()
            && n > min
        {
            topk.pop();
            topk.push(Reverse(n));
        }
    }
    let mut result: Vec<i32> = topk.into_iter().map(|Reverse(v)| v).collect();
    result.sort_unstable();
    println!("  Top {k} 最大值: {result:?}");

    section("peek / len / is_empty");
    let heap: BinaryHeap<i32> = vec![1, 5, 3].into_iter().collect();
    println!("  peek = {:?}", heap.peek());
    println!("  len = {}, is_empty = {}", heap.len(), heap.is_empty());
}

/// 任务：演示「带优先级的元素」（需 Ord，按 priority 排序）。
#[derive(Debug, Eq, PartialEq)]
struct Task {
    priority: i32,
    name: &'static str,
}

// 让 Task 的排序「只看 priority」（大的优先）。
impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // 注意：堆是最大堆，priority 越大越靠前 → 直接按 priority 降序。
        // 这里用「priority 优先」比较，name 仅作 tiebreaker。
        self.priority
            .cmp(&other.priority)
            .then(self.name.cmp(other.name))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
