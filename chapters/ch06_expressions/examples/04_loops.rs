//! 6.4 循环：`while` / `for` / `loop`
//!
//! 关键结论：
//! - 4 种循环：`while`、`while let`、`loop`、`for`。
//! - `while`/`for` 的值总是 `()`；`loop` 可携带值（与 `break value` 配合）。
//! - `for x in iterable`：会消耗 iterable（按值迭代），传引用则按引用迭代。
//! - 范围 `a..b`（半开）、`a..=b`（闭区间）、`..b`、`a..`、`..`（完整）。
//!
//! 运行：`cargo run -p ch06_expressions --example 04_loops`

use ch06_expressions::section;

fn main() {
    section("while：经典条件循环");
    let mut countdown = 3;
    while countdown > 0 {
        println!("  {countdown}...");
        countdown -= 1;
    }
    println!("  launch!");

    section("for + 半开范围 0..5（含 0 不含 5）");
    for i in 0..5 {
        print!("  {i}");
    }
    println!();

    section("for + 闭区间 0..=3（含 0 也含 3）");
    for i in 0..=3 {
        print!("  {i}");
    }
    println!();

    section("for 按值迭代会消耗容器");
    let names: Vec<String> = vec!["alice".into(), "bob".into()];
    for n in names {
        // n: String，所有权被移走
        println!("  owned: {n}");
    }
    // println!("{:?}", names); // ❌ names 已被消耗

    section("for &T 迭代引用 —— 元素不被移动");
    let names: Vec<String> = vec!["alice".into(), "bob".into()];
    for s in &names {
        // s: &String
        println!("  ref: {s} @ {:p}", s);
    }
    println!("  names 仍然可用：{names:?}");

    section("for &mut T 迭代可变引用 —— 就地修改");
    let mut nums: Vec<i32> = vec![1, 2, 3];
    for n in &mut nums {
        *n *= 10;
    }
    println!("  nums = {nums:?}");

    section("loop + break value：携带返回值");
    // 找到给定数字的平方根（若是完全平方数）
    // 注意：`break value` 只能在 `loop` 中使用；要跨出内层循环带值，
    // 必须用「循环标签」`'outer:` + `break 'outer value`。
    let n: i32 = 64; // 模拟外部数据
    #[allow(clippy::never_loop, reason = "演示 break 'outer 的语义")]
    let sqrt: i32 = 'outer: loop {
        let mut i: i32 = 1;
        loop {
            let sq = i * i;
            if sq == n {
                break 'outer i; // ← 直接跳出外层 loop，并带值 i
            }
            if sq > n {
                break 'outer 0; // 未找到完全平方数
            }
            i += 1;
        }
        // 由于上面的 break 'outer，这里永远到不了
    };
    println!("  sqrt(第一个匹配) = {sqrt}");

    section("Rust 没有 while(true)：用 loop 表达「永不退出」");
    // `loop` 的类型可以「发散」（不返回），从而绕开「函数必须返回 i32」的检查：
    let forever = || -> i32 {
        let mut i = 0;
        loop {
            i += 1;
            if i >= 3 {
                return i;
            }
        }
    };
    println!("  forever() = {}", forever());
}
