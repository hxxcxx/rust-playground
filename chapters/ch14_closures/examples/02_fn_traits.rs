//! 14.2 Fn / FnMut / FnOnce：把闭包作为函数参数
//!
//! 关键结论：
//! - 三个标准 trait 描述「闭包能怎么调用」：
//!   * `Fn(&self)`        —— 不改变捕获的环境（可多次调用，最严格）
//!   * `FnMut(&mut self)` —— 可改变捕获的环境（可多次调用）
//!   * `FnOnce(self)`     —— 消耗捕获的环境（只能调用一次，最宽松）
//! - 层级：Fn ⊂ FnMut ⊂ FnOnce（实现 Fn 的也实现 FnMut 和 FnOnce）。
//! - 函数/闭包参数用哪个？
//!   * 只读 → Fn
//!   * 需要修改捕获值 → FnMut
//!   * 需要消耗捕获值（如移动它） → FnOnce
//! - 普通函数 `fn` 也实现了这三个 trait（可作为参数传递）。
//!
//! 运行：`cargo run -p ch14_closures --example 02_fn_traits`

use ch14_closures::section;

fn main() {
    section("Fn：不改变环境，可多次调用");
    let greeting = String::from("Hi");
    // 闭包只读借用 greeting → 实现 Fn。
    let say = || println!("  {greeting}, world");
    call_fn(say);
    call_fn(|| println!("  直接传字面量也行"));

    section("FnMut：修改捕获的环境");
    let mut counter = 0;
    // 闭包修改 counter → 实现 FnMut（不实现 Fn）。
    let mut bump = || {
        counter += 1;
        counter
    };
    call_fn_mut(&mut bump);
    call_fn_mut(&mut bump);
    println!("  最终 counter = {counter}");

    section("FnOnce：消耗捕获的环境（只能调一次）");
    let data = vec![1, 2, 3];
    // 闭包把 data 移动进返回值 → 实现 FnOnce。
    let consume = || {
        println!("  消耗 data: {data:?}");
        data.len()
    };
    let len = call_fn_once(consume);
    println!("  返回 len = {len}");
    // consume 已被消耗，不能再用。

    section("普通 fn 函数也实现 Fn/FnMut/FnOnce");
    // 命名函数 greet 可以传给接受 Fn() 的参数（签名匹配）。
    call_fn(greet);
    let nums = [1, 2, 3];
    // 标准库的 map 接受 FnMut —— 这里传命名函数 double(&i32)->i32。
    let doubled: Vec<i32> = nums.iter().map(double).collect();
    println!("  map(double) = {doubled:?}");

    section("trait 选择的影响：能用 Fn 就别用 FnMut");
    // Fn 最严格 → 接受范围最小，但调用方限制最少（可反复调用、可多线程共享）。
    // 设计 API：优先 Fn，不够再升级 FnMut，最后才 FnOnce。
    println!("  优先级：Fn > FnMut > FnOnce");
}

/// 接受任意 Fn 闭包并调用一次。
fn call_fn<F: Fn()>(f: F) {
    f();
}

/// 接受 FnMut（需要 &mut，因为可能修改环境）。
fn call_fn_mut<F: FnMut() -> i32>(f: &mut F) {
    let r = f();
    println!("  FnMut 返回 = {r}");
}

/// 接受 FnOnce（按值接收，消耗它）。
fn call_fn_once<F: FnOnce() -> usize>(f: F) -> usize {
    f()
}

/// 一个普通命名函数（实现 Fn）。
fn double(n: &i32) -> i32 {
    n * 2
}

/// 不带参数的命名函数，用于演示 call_fn。
fn greet() {
    println!("  hello from greet");
}
