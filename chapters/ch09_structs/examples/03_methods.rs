//! 9.3 方法与关联函数：impl 块
//!
//! 关键结论：
//! - 方法定义在 `impl Type { ... }` 块里，不在 struct 定义里。
//! - self 参数的几种形式：
//!   * `&self`     → 共享引用（只读）
//!   * `&mut self` → 可变引用（可改 self）
//!   * `self`      → 按值（消耗 self，拿到所有权）
//!   * `Box<Self>`/`Rc<Self>`/`Arc<Self>` → 智能指针（少见，用于所有权操作）
//! - 类型关联函数：没有 self 参数的 fn，类似「静态方法」（如 `Vec::new()`）。
//! - `Self` 是当前类型的别名；可省略完整类型名。
//! - 方法调用自动借用：`q.push(x)` 等价于 `(&mut q).push(x)`。
//!
//! 运行：`cargo run -p ch09_structs --example 03_methods`

use ch09_structs::{Queue, section};

fn main() {
    section("类型关联函数（无 self 参数，类似静态方法）");
    let mut q: Queue<char> = Queue::new();
    println!("  Queue::new() → q.is_empty() = {}", q.is_empty());

    section("方法调用自动借用：q.push('0') 等价于 (&mut q).push('0')");
    q.push('0');
    q.push('1');
    q.push('∞');
    println!("  push 三个元素后 is_empty = {}", q.is_empty());

    section("&mut self 方法：可修改 self");
    let first = q.pop();
    println!("  q.pop() = {first:?}");

    section("self（按值）方法：消耗 self，拿到所有权");
    q.push('X');
    let (older, younger) = q.split();
    println!("  split() → older={older:?}, younger={younger:?}");
    // q 现在已经无效（被消耗）

    section("Default trait：Queue 实现了 Default，可用 Default::default()");
    let mut q2: Queue<i32> = Default::default();
    q2.push(10);
    q2.push(20);
    println!("  q2.pop() = {:?}, pop() = {:?}", q2.pop(), q2.pop());

    section("方法调用自动从 Box/Rc/Arc 解引用");
    let mut bq = Box::new(Queue::<&str>::new());
    // Queue::push 期望 &mut Queue，但 bq 是 Box<Queue>
    // Rust 会自动从 Box 借用 &mut —— 不需要写 (*bq).push(...)
    bq.push("from box");
    println!("  Box<Queue> 调用方法: pop() = {:?}", bq.pop());
}
