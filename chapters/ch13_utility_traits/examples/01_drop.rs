//! 13.1 Drop —— 值离开作用域时自动调用
//!
//! 关键结论：
//! - `Drop` 只有一个方法 `drop(&mut self)`，在值「离开作用域」时自动调用。
//! - 用途：释放堆内存、关闭文件/网络连接、释放锁、打印日志、回滚事务。
//! - 析构顺序：与「声明顺序相反」（栈式 / LIFO）。
//! - 不能手动调用 `x.drop()`（编译器禁止）；要提前析构用 `std::mem::drop(x)`。
//! - 实现了 Drop 的类型通常「不应该」再实现 Copy（Copy 暗示「无资源需要释放」）。
//!
//! 运行：`cargo run -p ch13_utility_traits --example 01_drop`

use ch13_utility_traits::{Droppy, section};

fn main() {
    section("Drop 自动调用：函数返回时按声明逆序析构");
    make_and_drop();
    println!("  （make_and_drop 已返回，所有 Droppy 都已析构）");

    section("析构顺序：LIFO（后构造的先析构）");
    {
        let _a = Droppy::new("A");
        let _b = Droppy::new("B");
        let _c = Droppy::new("C");
        println!("  --- 块结束 ---");
        // 析构顺序：C → B → A（与构造相反）
    }

    section("提前析构：std::mem::drop(...)");
    {
        let g = Droppy::new("guard");
        println!("  业务逻辑开始");
        // 不能写 g.drop(); —— 编译器禁止手动调用 drop
        drop(g); // 在这里立即析构 g（提前释放资源）
        println!("  业务逻辑继续（g 已析构）");
        // 块结束时没有 g 要析构了
    }

    section("Drop 用于「守卫」：确保清理逻辑一定执行");
    let _timer = Timer::start("耗时操作");
    do_some_work();
    // 即使 do_some_work 出错（这里没演示 panic），Timer 的 Drop 也会运行。
    println!("  工作完成");

    section("Option + take：有条件地释放值");
    let mut resource = Some(Droppy::new("R"));
    // take 把内部值拿出来（替换为 None），原来的 Droppy 在 take 后立即析构。
    let _taken = resource.take();
    println!("  resource 现在是 None: {resource:?}");
    println!("  （块结束时只剩 None，不会再析构 Droppy(R)）");
}

/// 演示「带 Drop 的守卫」：测量一段代码的耗时。
struct Timer {
    label: &'static str,
    start: std::time::Instant,
}

impl Timer {
    fn start(label: &'static str) -> Self {
        println!("  ⏱  [{label}] 开始");
        Self {
            label,
            start: std::time::Instant::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        println!("  ⏱  [{}] 结束，耗时 {:?}", self.label, elapsed);
    }
}

fn make_and_drop() {
    let _x = Droppy::new("X");
    let _y = Droppy::new("Y");
    // 函数返回时：先析构 Y，再析构 X
}

fn do_some_work() {
    // 模拟一点工作
    let _sum: u64 = (0..1000).map(|i| i as u64).sum();
}
