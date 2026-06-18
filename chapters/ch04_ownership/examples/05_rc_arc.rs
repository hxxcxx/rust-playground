//! 4.4 Rc 和 Arc：共享所有权
//!
//! 关键结论：
//! - `Rc<T>` / `Arc<T>` 是引用计数指针：克隆它只是增加计数，多个指针指向同一份堆数据。
//! - 当最后一个 `Rc`/`Arc` 被 drop 时，被指向的 `T` 才会被释放（类似 Python 的引用计数）。
//! - `Rc`：单线程，更快；`Arc`：原子引用计数，可跨线程共享。
//! - `Rc`/`Arc` 指向的值是「不可变」的 —— Rust 的核心安全保证：不会「同时被共享且可变」。
//! - 引用计数循环会导致内存泄漏；用 `std::rc::Weak` 弱引用打破环。
//!
//! 运行：`cargo run -p ch04_ownership --example 05_rc_arc`

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

use ch04_ownership::section;

/// 演示：`Rc::clone` 只是复制指针+增加引用计数，不会复制底层 String。
fn rc_basics() {
    // Rc::new 在堆上分配 String + 引用计数
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = Rc::clone(&s); // 不复制 String，只增加计数 → 2
    let u: Rc<String> = Rc::clone(&s); // 计数 → 3

    println!(
        "strong_count = {} (s/t/u 三个指针指向同一份 String)",
        Rc::strong_count(&s)
    );

    // 任何一个 Rc<String> 都可以直接调用 String 的方法（Deref 自动解引用）
    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{u} are quite chewy, almost bouncy, but lack flavor");
}

/// 演示：`Rc<T>` 指向的值不可变。任何尝试修改都会被拒绝。
#[allow(dead_code)]
fn rc_is_immutable() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    // s.push_str(" noodles"); // ❌ cannot borrow data in an `Rc` as mutable
    println!("Rc 指向的值不可变：s = {s}");
}

/// 演示：用 `Rc` 让多个所有者共享同一份节点（链表示意图）。
/// 没有 `Rc` 的话，每个节点只能有一个所有者，无法构造「多个父节点指向同一子节点」。
#[allow(dead_code)]
fn shared_subtree() {
    // 一个共享的叶子节点
    let leaf = Rc::new("leaf".to_string());

    // 两个父节点都持有 leaf 的引用计数指针
    let parent_a = vec![Rc::clone(&leaf)];
    let parent_b = vec![Rc::clone(&leaf)];

    println!(
        "leaf 的 strong_count = {}（parent_a 和 parent_b 都引用它）",
        Rc::strong_count(&leaf)
    );
    println!("parent_a = {parent_a:?}");
    println!("parent_b = {parent_b:?}");
    // leaf 不会在 parent_a / parent_b 任一离开作用域时被释放；
    // 只有两者都 drop 后，引用计数归零，leaf 才会被释放。
    drop(parent_a);
    println!(
        "drop(parent_a) 后 leaf strong_count = {}",
        Rc::strong_count(&leaf)
    );
    drop(parent_b);
    // 此时 leaf 也被释放（不能再访问 leaf）
}

/// 演示：跨线程共享必须用 `Arc`（原子引用计数），`Rc` 不行。
fn arc_across_threads() {
    // 用 Arc 包装一个不可变的共享值
    let a: Arc<Vec<i32>> = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for _ in 0..3 {
        let a_clone = Arc::clone(&a); // 原子地增加引用计数
        handles.push(thread::spawn(move || {
            // 每个线程都能安全地只读访问同一份 Vec
            println!(
                "线程看到: {a_clone:?}, sum = {}",
                a_clone.iter().sum::<i32>()
            );
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("所有线程结束后，a 仍然有效: {a:?}");
}

fn main() {
    section("Rc 基础：clone 只增加引用计数");
    rc_basics();

    section("Rc 指向的值不可变");
    rc_is_immutable();

    section("Rc 实现多所有者：多个父节点共享同一个子节点");
    shared_subtree();

    section("Arc：跨线程共享所有权");
    arc_across_threads();

    section("总结");
    println!("Rc/Arc 是「放宽单一所有者」的机制：用引用计数换共享。");
    println!("代价：值不可变；且要小心循环引用导致内存泄漏（用 Weak 打破环）。");
}
