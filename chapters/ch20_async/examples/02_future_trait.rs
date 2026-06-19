//! 20.2 Future trait / poll / Poll / Pin —— 手写 Future
//!
//! 关键结论：
//! - `Future` 是一个 trait，核心方法：
//!   `fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Output>`
//! - `Poll<T>` 是个枚举：
//!   * `Ready(v)` —— 完成了，结果是 v。
//!   * `Pending` —— 还没好，稍后再 poll。
//! - `Pin`：保证「自引用」的 Future 不会被移动（async 状态机需要）。
//! - `Context`：包含 `Waker`，Future 用它告诉运行时「我好了，再来 poll 我」。
//! - Future 是「惰性」的：创建它什么都不做，必须 poll 才会推进。
//! - poll 的契约：
//!   * 返回 Pending 时，必须保证「未来某刻会调用 cx.waker().wake()」。
//!   * 否则这个 Future 永远不会再被 poll（被遗忘）。
//!
//! 运行：`cargo run -p ch20_async --example 02_future_trait`

use ch20_async::{block_on, delay, ready, section};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

fn main() {
    section("Future 是惰性的：创建时不执行");
    let f = ready(42); // 创建 Future，但还没 poll
    println!("  创建了 Future（还没执行）");
    // 用我们的 block_on 驱动它。
    let result = block_on(f);
    println!("  block_on 后得到: {result}");

    section("手写 Future：理解 poll 机制");
    // 手写一个「计数器」Future：poll N 次后才返回。
    let counter = CountDown::new(3);
    let result = block_on(counter);
    println!("  CountDown 结果: {result:?}");

    section("Poll::Pending 与重试机制");
    // Delay 第一次 poll 返回 Pending，之后才 Ready。
    let start = std::time::Instant::now();
    block_on(async {
        delay(Duration::from_millis(30)).await;
    });
    println!("  Delay 等了 {:?}（poll 多次直到 Ready）", start.elapsed());

    section("Future 是状态机：async fn 的本质");
    // async fn 被编译成「实现 Future 的状态机」。
    // 每个 .await 点是一个状态切换；状态保存在 Future 对象里（不占线程栈）。
    // 这就是「零成本」：没有堆分配、没有 GC、没有虚函数（除非用 Box<dyn Future>）。
    let f = example_async_fn(); // 这个 Future 是个状态机
    let result = block_on(f);
    println!("  async fn 结果: {result}");

    section("组合多个 Future：async 块");
    let result = block_on(async {
        let a = ready(10).await;
        let b = ready(20).await;
        a + b // async 块的返回值就是 Future 的 Output
    });
    println!("  async 块: 10 + 20 = {result}");

    section("Future 的大小：编译期已知（状态机）");
    // Future 的大小取决于它「捕获了什么 + 有几个 await 状态」。
    fn size_of_future<T>(_f: T) -> usize {
        std::mem::size_of::<T>()
    }
    let f1 = ready(5_i32);
    println!("  ready(i32) 的大小: {} 字节", size_of_future(f1));
    let f2 = async { let a = ready(1).await; let b = ready(2).await; a + b };
    println!("  async{{1+2}} 的大小: {} 字节（含两个 await 状态）", size_of_future(f2));

    section("Future 不自动运行：必须有 executor");
    // 这跟其它语言不同：Rust 的 Future 创建后不会自己跑。
    // 必须交给 executor（block_on / tokio::spawn / #[tokio::main]）来 poll。
    println!("  Future 创建 → 什么都不做");
    println!("  executor poll → 才推进");
    println!("  （Python/JS 的 async 是运行时自动调度的；Rust 把它留给库）");

    section("Pin 的作用：防止自引用 Future 被移动");
    // async 状态机可能「引用自己的字段」（跨 await）→ 自引用。
    // 移动自引用对象会破坏内部指针 → Pin 保证它不被移动。
    // Pin<&mut Future> 让 poll 安全 —— Future 不能被 move 走。
    println!("  Pin<&mut T>：保证 T 不会被移动");
    println!("  异步状态机跨 await 引用自己 → 需要 Pin");
    println!("  （Box::pin / Pin::new 是常见构造方式）");
}

/// 手写的「倒计时」Future：poll N 次后才返回 Ready。
///
/// 演示「状态保存在 Future 里」+「多次 poll」。
struct CountDown {
    remaining: u32,
}

impl CountDown {
    fn new(n: u32) -> Self {
        Self { remaining: n }
    }
}

impl Future for CountDown {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining == 0 {
            Poll::Ready("倒计时结束！".into())
        } else {
            println!("    [CountDown] 还剩 {} 次", self.remaining);
            self.remaining -= 1;
            // 安排自己立即被再 poll（教学简化）。
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// 一个简单的 async 函数（演示它编译成状态机）。
async fn example_async_fn() -> i32 {
    let a = ready(1).await;
    let b = ready(2).await;
    let c = ready(3).await;
    a + b + c
}
