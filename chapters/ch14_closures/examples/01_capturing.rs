//! 14.1 闭包捕获环境变量：& / &mut / move
//!
//! 关键结论：
//! - 闭包 `|...| body` 能「看到」定义点周围的变量（捕获）。
//! - 编译器按「最小权限」自动决定捕获方式：
//!   * 只读访问 → 借用 `&T`
//!   * 修改访问 → 可变借用 `&mut T`
//!   * 需要「拿走所有权」或离开作用域使用 → 用 `move` 强制移动
//! - `move` 关键字：强制闭包「按值」获取捕获变量的所有权（常用于线程）。
//! - 闭包捕获会「延长」被借用变量的生命周期到闭包销毁。
//!
//! 运行：`cargo run -p ch14_closures --example 01_capturing`

use ch14_closures::section;

fn main() {
    section("闭包借用捕获（最常见，只读）");
    let x = 10;
    // 闭包捕获了 x 的引用（&x），因为只读访问。
    let add = |y| x + y;
    println!("  add(5) = {}", add(5));
    // x 还能用（闭包只是借用，且立即返回，借用结束）。
    println!("  x 仍然是 {}", x);

    section("闭包可变借用捕获（修改环境）");
    let mut count = 0;
    // 把闭包放进一个块 —— 块结束时闭包销毁，可变借用结束，count 重新可用。
    {
        // 这个闭包会修改 count → 可变借用。
        // 注意：闭包用 mut 声明，因为它持有 &mut 并改变状态。
        let mut increment = || {
            count += 1;
        };
        increment();
        increment();
        increment();
        // 块结束时 increment 销毁 → count 的可变借用释放。
    }
    println!("  count = {count}（块结束后才能访问 count）");

    section("move 关键字：强制按值捕获（移动）");
    let text = String::from("hello");
    // 没有 move：闭包借用 text，text 还能用。
    // 有 move：闭包拿走 text 的所有权。
    let printer = move || {
        println!("  闭包持有 text: {text}");
    };
    printer();
    // println!("{text}"); // ❌ text 已被 move 进闭包
    println!("  （text 已移入闭包，外部无法访问）");

    section("move 在线程中的典型用途");
    let data = vec![1, 2, 3];
    // 线程闭包必须 move：主线程可能先结束，data 必须归子线程所有。
    let handle = std::thread::spawn(move || {
        println!("  子线程拿到 data: {data:?}");
        data.len()
    });
    let len = handle.join().unwrap();
    println!("  子线程返回 len = {len}");

    section("捕获 Copy 类型：移动 = 复制");
    let n = 42_i32; // i32 是 Copy
    let f = move || n;
    println!("  f() = {}", f());
    println!("  n 还能用 = {n}（i32 是 Copy，move 等于复制）");

    section("每个闭包有独特的匿名类型");
    let f1 = |x| x + 1;
    let f2 = |x| x + 1;
    // f1 和 f2 类型「不同」（即使代码一样）—— 这是为什么闭包常作泛型参数。
    println!("  f1(10) = {}, f2(10) = {}", f1(10), f2(10));
    // 下面会编译失败（类型不同）：
    //   let v: Vec<fn(i32)->i32> = vec![f1, f2];
    println!("  （f1/f2 是不同的匿名类型，不能放进同一 Vec<具体类型>）");
}
