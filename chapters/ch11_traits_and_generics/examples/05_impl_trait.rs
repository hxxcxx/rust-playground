//! 11.5 impl Trait —— 参数位置与返回位置
//!
//! 关键结论：
//! - `impl Trait` 在「参数位置」是 trait bound 的语法糖：
//!   `fn f(x: impl Trait)`  ≡  `fn f<T: Trait>(x: T)`
//! - `impl Trait` 在「返回位置」用于「隐藏具体类型」：
//!   `fn make() -> impl Trait` 返回「某种实现了 Trait 的类型」，但调用方看不到具体类型。
//!   好处：可以返回「闭包」「map/filter 链」「impl 块里私有的类型」而不暴露内部。
//!   限制：只能返回「一种」具体类型（不能在 if/else 返回不同的具体类型）。
//! - 静态分发 vs 动态分发：
//!   * impl Trait   → 静态（编译期单态化，性能好，但调用方拿到的是具体类型）
//!   * dyn Trait    → 动态（运行期 vtable，可装异构，但有间接调用开销）
//!
//! 运行：`cargo run -p ch11_traits_and_generics --example 05_impl_trait`

use ch11_traits_and_generics::{Circle, IsShape, Rectangle, section};

// =====================================================================
// 参数位置的 impl Trait —— 等价于 trait bound
// =====================================================================

/// 接收任意 IsShape，打印它的名字。
/// 等价于 `fn show<T: IsShape>(s: &T)`，只是写法更短。
fn show(s: &impl IsShape) {
    println!("  {} 面积={:.2}", s.name(), s.area());
}

/// 多个 impl Trait 参数 —— 注意它们是「互相独立」的类型。
/// `fn add(a: impl Add, b: impl Add)` 中 a 和 b 可以是不同类型。
fn show_two(a: &impl IsShape, b: &impl IsShape) {
    println!(
        "  {} + {} = 总面积 {:.2}",
        a.name(),
        b.name(),
        a.area() + b.area()
    );
}

// =====================================================================
// 返回位置的 impl Trait —— 隐藏具体类型
// =====================================================================

/// 返回一个 IsShape，但调用方看不到是 Circle 还是 Rectangle。
/// 好处：内部可以换成别的实现而不破坏 API。
fn make_unit_circle() -> impl IsShape {
    Circle { radius: 1.0 }
}

/// 返回一个闭包 —— 没有 impl Trait 很难写（闭包类型是编译器生成的匿名类型）。
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // 闭包类型没有名字，只能用 impl Fn(...) -> ... 来描述。
    move |x| x + n
}

/// 返回 iterator 适配器链 —— 这是最常见的 impl Trait 用途。
/// map/filter 返回的是「匿名迭代器类型」，用 impl Iterator 描述。
fn evens_squared(up_to: i32) -> impl Iterator<Item = i32> {
    (0..up_to).filter(|x| x % 2 == 0).map(|x| x * x)
}

// 注意：返回 impl Trait 时，所有分支必须返回「同一种」具体类型。
// 下面这个函数会编译失败（故意注释）：
// fn pick_shape(big: bool) -> impl IsShape {
//     if big { Rectangle { width: 10.0, height: 10.0 } }  // 类型 A
//     else   { Circle { radius: 1.0 } }                   // 类型 B —— 报错！
// }
// 想要这种「条件返回不同类型」的语义 → 用 Box<dyn IsShape>。

fn main() {
    section("参数位置 impl Trait：trait bound 的简写");
    show(&Circle { radius: 2.0 });
    show(&Rectangle { width: 3.0, height: 4.0 });

    section("两个 impl Trait 参数可以是不同类型");
    show_two(&Circle { radius: 1.0 }, &Rectangle { width: 2.0, height: 3.0 });

    section("返回位置 impl Trait：隐藏具体类型");
    let s = make_unit_circle();
    // s 的具体类型对调用方是「隐藏」的，只知道它实现了 IsShape。
    println!("  make_unit_circle() → name={}, area={:.2}", s.name(), s.area());

    section("返回闭包：没有 impl Trait 几乎写不出");
    let add5 = make_adder(5);
    println!("  make_adder(5)(10) = {}", add5(10));
    println!("  make_adder(5)(20) = {}", add5(20));

    section("返回 iterator 适配器链");
    let v: Vec<i32> = evens_squared(8).collect();
    println!("  evens_squared(8) = {:?}", v);

    section("impl Trait（静态）vs Box<dyn Trait>（动态）对比");
    let shapes: Vec<Box<dyn IsShape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle { width: 2.0, height: 2.0 }),
    ];
    // 这里必须用 Box<dyn>，因为两种具体类型要塞进同一个 Vec。
    // 如果只有一个 Circle，就可以用 Vec<Circle>（静态、更快）。
    for s in &shapes {
        println!("  [dyn] {} 面积={:.2}", s.name(), s.area());
    }

    section("性能直觉");
    println!("  静态分发（impl Trait / 泛型）：编译期就确定，调用直接、可内联");
    println!("  动态分发（Box<dyn>）：运行期查 vtable，多一次间接跳转");
    println!("  选择标准：是否需要「一个容器装多种类型」？需要 → dyn；否则 → 泛型/impl");
}
