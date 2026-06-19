//! 第20章 异步编程（Asynchronous Programming）—— 共享类型与手写 Future 示例。
//!
//! 本章核心：理解 Rust 异步的「零成本」模型，以及 async/await 如何工作。
//!
//! - `Future` trait：核心是 `poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<T>`。
//!   * `Poll::Pending` —— 还没完成，会在准备好时用 cx 的 waker 唤醒。
//!   * `Poll::Ready(v)` —— 完成，返回结果 v。
//! - 异步函数 `async fn` 编译成「实现 Future 的状态机」—— 零堆分配、零间接调用。
//! - `.await` 就是「poll 这个 Future 直到 Ready」的语法糖。
//! - 异步必须有「运行时」（executor）来 poll 这些 Future —— Rust 标准库只提供 trait，
//!   运行时由第三方提供（tokio / async-std / smol）。
//! - 异步 vs 线程：异步用「协作式调度」（.await 时让出），单线程可处理上万并发；
//!   线程是「抢占式调度」，每个都有 OS 开销。
//!
//! 本章先用手写 Future + 手写执行器讲清「机制」，再用 tokio 讲「实战」。

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

// =======================================================================
// 工具函数
// =======================================================================

/// 打印带标题的分割线（与前几章风格一致）。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

// =======================================================================
// 例 1：手写一个 Future（最简单的「立即完成」）
// =======================================================================

/// 一个永远「立即就绪」的 Future：第一次 poll 就返回 Ready。
///
/// 演示 Future trait 最朴素的实现 —— 不依赖任何运行时。
pub struct Ready<T>(Option<T>);

impl<T> Ready<T> {
    pub fn new(value: T) -> Self {
        Self(Some(value))
    }
}

impl<T: Unpin> Future for Ready<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Ready<T> 是 Unpin（不含自引用），可以安全地拿到 &mut Self。
        // 用 get_mut 是 Pin 提供的安全 API（要求 Self: Unpin）。
        match self.get_mut().0.take() {
            Some(v) => Poll::Ready(v),
            None => panic!("Ready polled after completion"),
        }
    }
}

/// 构造一个立即就绪的 Future（类似 std 的 `std::future::ready`）。
pub fn ready<T>(value: T) -> Ready<T> {
    Ready::new(value)
}

// =======================================================================
// 例 2：手写「延迟」Future —— 真正会返回 Pending 的例子
// =======================================================================

/// 一个「延迟若干毫秒后完成」的 Future（用同步 sleep 模拟，教学用）。
///
/// 演示 Pending / Waker 机制：
/// - 第一次 poll：还没到时间 → 返回 Pending。
/// - 后续 poll：到时间了 → 返回 Ready。
///
/// 注意：这个实现用「轮询检查时间」简化了 Waker 机制；
/// 真实运行时会用 Waker 在到点时被「唤醒」（见 tokio::time::sleep）。
pub struct Delay {
    when: std::time::Instant,
}

impl Delay {
    pub fn for_duration(d: std::time::Duration) -> Self {
        Self {
            when: std::time::Instant::now() + d,
        }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if std::time::Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            // 还没到点：告诉运行时「我还没好」。
            // 这里用「安排自己稍后再被 poll」的简化策略（立即唤醒）。
            // 真实场景会注册一个定时器，到点调用 cx.waker().wake()。
            let waker = cx.waker().clone();
            waker.wake();
            Poll::Pending
        }
    }
}

/// 构造一个延迟 Future。
pub fn delay(d: std::time::Duration) -> Delay {
    Delay::for_duration(d)
}

// =======================================================================
// 例 3：手写一个「最小执行器」—— 能 poll 任意 Future 到完成
// =======================================================================

/// 一个最小的 Future 执行器（block_on）：不断 poll 直到 Ready。
///
/// 这就是「运行时」的本质 —— 用 Waker 驱动 Future。
/// 真实的 tokio 比这复杂得多（多线程、IO 多路复用、定时器堆...）。
///
/// 用 `Box::pin` 把 Future pin 到堆上（安全地拿到 Pin<&mut F>）。
pub fn block_on<F: Future>(future: F) -> F::Output {
    // 用标准库的 noop waker（我们的 Delay 在返回 Pending 前会调用
    // waker.wake_by_ref() 自我唤醒，所以这里只需一个合法占位 waker）。
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);

    // Box::pin 把 future pin 到堆上，安全地拿到 Pin<Box<F>>。
    let mut future = Box::pin(future);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(out) => return out,
            Poll::Pending => {
                // 让出 CPU 一小会再重试（避免 100% 占用）。
                // 真实运行时会在这时去 poll 其它就绪的 Future。
                std::thread::yield_now();
            }
        }
    }
}
