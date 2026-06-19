//! 14.6 闭包的性能：零开销，与函数指针对比
//!
//! 关键结论：
//! - 闭包编译后 = 「匿名结构体（存捕获的变量）+ impl Fn 的方法」。
//! - 调用闭包 = 直接调用方法 —— 与普通函数一样快，**零开销**。
//! - 泛型 `F: Fn`（静态分发）会单态化，每个具体闭包生成一份代码 → 可内联优化。
//! - `Box<dyn Fn>` / `&dyn Fn`（动态分发）有一次间接调用，略慢但仍接近函数指针。
//! - `fn` 指针（裸函数指针）是最轻量的，但「不能捕获环境」。
//!
//! 运行：`cargo run -p ch14_closures --example 06_performance`

use ch14_closures::section;
use std::time::Instant;

fn main() {
    section("闭包 vs 函数：调用开销几乎相同");
    let n = 1_000_000;

    // 1) 普通函数
    let t = Instant::now();
    let mut sum = 0u64;
    for i in 0..n {
        sum = sum.wrapping_add(double_fn(i));
    }
    let fn_time = t.elapsed();
    println!("  普通函数 sum={sum}, 耗时 {fn_time:?}");

    // 2) 闭包（静态分发）
    let closure = |x: u64| x.wrapping_mul(2);
    let t = Instant::now();
    let mut sum = 0u64;
    for i in 0..n {
        sum = sum.wrapping_add(closure(i));
    }
    let cl_time = t.elapsed();
    println!("  闭包      sum={sum}, 耗时 {cl_time:?}");

    // 3) 函数指针 fn
    let ptr: fn(u64) -> u64 = double_fn;
    let t = Instant::now();
    let mut sum = 0u64;
    for i in 0..n {
        sum = sum.wrapping_add(ptr(i));
    }
    let ptr_time = t.elapsed();
    println!("  fn 指针   sum={sum}, 耗时 {ptr_time:?}");

    section("闭包捕获 vs 不捕获：开销取决于捕获内容");
    let factor = 3_u64;
    // 捕获 factor 的闭包 —— 编译为「持有 factor 的结构体」。
    let with_capture = move |x: u64| x.wrapping_mul(factor);
    let t = Instant::now();
    let mut sum = 0u64;
    for i in 0..n {
        sum = sum.wrapping_add(with_capture(i));
    }
    println!("  捕获闭包 sum={sum}, 耗时 {:?}", t.elapsed());

    section("Box<dyn Fn>：动态分发，多一次间接调用");
    let boxed: Box<dyn Fn(u64) -> u64> = Box::new(move |x| x.wrapping_mul(2));
    let t = Instant::now();
    let mut sum = 0u64;
    for i in 0..n {
        sum = sum.wrapping_add(boxed(i));
    }
    let dyn_time = t.elapsed();
    println!("  Box<dyn> sum={sum}, 耗时 {dyn_time:?}");

    section("fn 指针：不能捕获，但最轻量");
    // fn 指针就是一个地址，没有捕获的数据。
    let f: fn(u64) -> u64 = |x| x * 2; // 不捕获的闭包可强转为 fn 指针
    println!("  fn 指针(不捕获闭包转来) f(21) = {}", f(21));

    section("结论：性能排序（通常）");
    println!("  泛型闭包/函数 ≈ 不捕获闭包 ≈ fn 指针  （都极快，可内联）");
    println!("  Box<dyn Fn> 略慢（间接调用，难内联）");
    println!("  选择：不需要捕获 → fn；需要捕获 → 闭包；异构集合 → Box<dyn>");
}

/// 普通命名函数。
fn double_fn(x: u64) -> u64 {
    x.wrapping_mul(2)
}
