//! 9.4 泛型结构体 + 带生命周期参数的结构体
//!
//! 关键结论：
//! - `struct Queue<T> { ... }` —— 类型参数 T，使用时填具体类型。
//! - `impl<T> Queue<T>` —— 给「所有 T 的 Queue」实现方法。
//! - 也可以为特定类型实现：`impl Queue<f64> { fn sum() {...} }`。
//! - 带生命周期：`struct Extrema<'elt>` —— 字段引用的生命周期必须 ≥ 'elt。
//! - 单一生命周期可省略：`fn f(slice: &[i32]) -> Extrema<'_>`。
//! - `Self` 关键字：在 impl 块中表示「正在实现方法的类型」。
//!
//! 运行：`cargo run -p ch09_structs --example 04_generics_lifetimes`

use ch09_structs::{Queue, find_extrema, section};

fn main() {
    section("泛型结构体：Queue<char>");
    let mut q1: Queue<char> = Queue::new();
    q1.push('a');
    q1.push('b');
    println!("  q1.pop() = {:?}, pop() = {:?}", q1.pop(), q1.pop());

    section("同一个 Queue 类型可存储不同元素类型");
    let mut q2: Queue<i32> = Queue::new();
    q2.push(1);
    q2.push(2);
    println!("  q2.pop() = {:?}, pop() = {:?}", q2.pop(), q2.pop());

    let mut q3: Queue<String> = Queue::new();
    q3.push("hello".into());
    q3.push("world".into());
    println!("  q3.pop() = {:?}, pop() = {:?}", q3.pop(), q3.pop());

    section("为特定类型 Queue<f64> 实现额外方法");
    // Queue<f64>::sum 在 lib.rs 中实现（孤儿规则要求 inherent impl 在定义类型的 crate 内）
    let mut q: Queue<f64> = Queue::new();
    q.push(1.5);
    q.push(2.5);
    q.push(3.0);
    println!("  Queue<f64>::sum() = {}", q.sum());

    section("带生命周期参数的结构体");
    let a = [0, -3, 0, 15, 48];
    let extrema = find_extrema(&a);
    println!(
        "  least = {}, greatest = {}",
        extrema.least, extrema.greatest
    );
    // 结构体的两个引用共享同一个生命周期 'elt，不能超过 a 的生命周期

    section("生命周期省略规则");
    // 当返回类型的生命周期明显与参数相同时，可省略为 `'_`
    let extrema = find_extrema(&[1, 2, 3]);
    println!(
        "  省略写法同样工作: least={}, greatest={}",
        extrema.least, extrema.greatest
    );
}
