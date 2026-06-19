//! 14.3 返回闭包：impl Fn / Box<dyn Fn>
//!
//! 关键结论：
//! - 闭包类型是「匿名的」，没有名字，无法直接写返回类型。
//! - 返回闭包的两种方式：
//!   * `-> impl Fn(...) -> ...`：静态分发，隐藏具体类型（首选）。
//!   * `-> Box<dyn Fn(...) -> ...>`：动态分发，用于「不同分支返回不同类型」。
//! - 用 impl Trait 返回闭包，要求所有分支返回「同一种」闭包类型。
//! - 「闭包工厂」：返回的闭包常常捕获了函数的参数（用 move）。
//!
//! 运行：`cargo run -p ch14_closures --example 03_returning`

use ch14_closures::section;

fn main() {
    section("返回闭包：impl Fn");
    let add5 = make_adder(5);
    println!("  make_adder(5)(10) = {}", add5(10));
    let add100 = make_adder(100);
    println!("  make_adder(100)(1) = {}", add100(100));

    section("返回捕获多个变量的闭包");
    // combine(2, 3) 返回一个闭包，对输入做 (x + a) * b
    let calc = combine(2, 3);
    println!("  combine(2,3)(4) = (4+2)*3 = {}", calc(4));

    section("返回的闭包可用于 map/filter 链");
    let nums = [1, 2, 3, 4, 5];
    // 把 make_adder 产出的闭包喂给 map。
    // 注意 iter() 产出 &i32，而 add10 接收 i32 → 用 |&x| 先解引用。
    let add10 = make_adder(10);
    let result: Vec<i32> = nums.iter().map(|&x| add10(x)).collect();
    println!("  map(make_adder(10)) = {result:?}");

    section("不同分支返回不同类型 → Box<dyn Fn>");
    // impl Fn 要求所有分支同类型；这里 if/else 返回不同闭包 → 用 Box<dyn Fn>。
    let f = make_op("+");
    println!("  make_op(\"+\")(3,4) = {}", f(3, 4));
    let g = make_op("*");
    println!("  make_op(\"*\")(3,4) = {}", g(3, 4));

    section("返回闭包 + 捕获可变状态（FnMut）");
    let mut counter = make_counter();
    println!("  调 3 次: {}, {}, {}", counter(), counter(), counter());

    section("闭包返回闭包（嵌套）");
    let multiplier = make_multiplier(3);
    let m5 = multiplier(5); // m5 是个闭包：把输入 × 3 × 5
    println!("  make_multiplier(3)(5)(2) = {}", m5(2));
}

/// 闭包工厂：返回一个「加 n」的闭包。
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // move：把 n 按值捕获进闭包（n 是 Copy，相当于复制）。
    move |x| x + n
}

/// 返回捕获两个变量的闭包：(input + a) * b
fn combine(a: i32, b: i32) -> impl Fn(i32) -> i32 {
    move |x| (x + a) * b
}

/// 不同分支返回「不同闭包类型」→ 必须 Box<dyn Fn>。
fn make_op(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    match op {
        "+" => Box::new(move |a, b| a + b),
        "*" => Box::new(move |a, b| a * b),
        "-" => Box::new(move |a, b| a - b),
        _ => Box::new(move |a, b| 0),
    }
}

/// 返回 FnMut：闭包内部维护一个递增的计数器。
fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

/// 返回「返回闭包的闭包」：make_multiplier(m) → 一个闭包 n → 一个闭包 x → x * m * n
fn make_multiplier(m: i32) -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32> {
    move |n| {
        let m = m;
        Box::new(move |x| x * m * n)
    }
}
