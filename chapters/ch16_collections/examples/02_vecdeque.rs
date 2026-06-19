//! 16.2 VecDeque<T> —— 双端队列
//!
//! 关键结论：
//! - VecDeque 是「环形缓冲区」：头尾两端 push/pop 都是 O(1)。
//! - 适合：队列（FIFO）、栈（LIFO）、滑动窗口、工作池。
//! - API 与 Vec 类似，多了 push_front/pop_front。
//! - LinkedList 极少用：Rust 里几乎总用 VecDeque 或 Vec 替代。
//!
//! 运行：`cargo run -p ch16_collections --example 02_vecdeque`

use ch16_collections::section;

fn main() {
    section("基本：头尾都可 push/pop");
    let mut dq = std::collections::VecDeque::new();
    dq.push_back(2);
    dq.push_back(3);
    dq.push_front(1); // 头部插入
    dq.push_front(0);
    println!("  {dq:?}（0,1,2,3）");
    println!("  pop_front() = {:?}", dq.pop_front()); // 0
    println!("  pop_back()  = {:?}", dq.pop_back()); // 3
    println!("  剩余: {dq:?}");

    section("用 VecDeque 实现 FIFO 队列");
    let mut queue: std::collections::VecDeque<&str> = std::collections::VecDeque::new();
    queue.push_back("任务A");
    queue.push_back("任务B");
    queue.push_back("任务C");
    // 从头部取（先进先出）。
    while let Some(task) = queue.pop_front() {
        println!("    处理: {task}");
    }

    section("用 VecDeque 实现栈（LIFO）");
    let mut stack: std::collections::VecDeque<i32> = std::collections::VecDeque::new();
    stack.push_back(1);
    stack.push_back(2);
    stack.push_back(3);
    // 从尾部取（后进先出）。
    while let Some(top) = stack.pop_back() {
        println!("    弹出: {top}");
    }

    section("滑动窗口：用 VecDeque 维护最近 N 个元素");
    let data = [1, 2, 3, 4, 5, 6, 7];
    let window_size = 3;
    let mut window: std::collections::VecDeque<i32> = std::collections::VecDeque::new();
    for &x in &data {
        window.push_back(x);
        if window.len() > window_size {
            window.pop_front(); // 超出窗口大小，弹出最旧的
        }
        println!("    进入 {x} → 窗口: {window:?}");
    }

    section("Vec ↔ VecDeque 互转");
    let v = vec![1, 2, 3];
    let dq: std::collections::VecDeque<i32> = v.into_iter().collect();
    println!("  Vec → VecDeque: {dq:?}");
    let v: Vec<i32> = dq.into_iter().collect();
    println!("  VecDeque → Vec: {v:?}");

    section("rotate：旋转（头尾搬运）");
    let mut dq: std::collections::VecDeque<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    println!("  原始: {dq:?}");
    dq.rotate_left(2); // 整体左移 2：[3,4,5,1,2]
    println!("  rotate_left(2): {dq:?}");
    dq.rotate_right(1); // 右移 1
    println!("  rotate_right(1): {dq:?}");

    section("LinkedList：几乎不用，了解即可");
    // Rust 的 LinkedList 是双向链表，但：
    // - 缓存不友好（节点散布在堆上）
    // - 大多数场景 Vec/VecDeque 更快
    // - 只有「需要频繁在中间 O(1) 插入删除」且「持有游标」时才考虑。
    let mut list = std::collections::LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(0);
    for x in &list {
        println!("    {x}");
    }
    println!("  （日常开发 99% 用 Vec，几乎不需要 LinkedList）");
}
