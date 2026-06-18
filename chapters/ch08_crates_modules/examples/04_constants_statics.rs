//! 8.4 常量与静态变量：const vs static
//!
//! 关键结论：
//! - `const NAME: T = expr;` —— 编译期常量，每次使用都被内联（类似 C 的 #define）。
//! - `static NAME: T = expr;` —— 进程级单例，有固定内存地址，所有线程共享。
//! - 二者都必须是 `Sync`（线程安全）类型 —— 默认值必须是常量表达式。
//! - 不可变 static 是线程安全的；可变 static (`static mut`) 本质上不安全，
//!   safe 代码完全不能使用 —— 因为 Rust 无法保证独占访问。
//! - 替代可变全局状态：用 `Mutex<T>`/`AtomicXxx`/`thread_local!`（见第 19 章）。
//!
//! 运行：`cargo run -p ch08_crates_modules --example 04_constants_statics`

use ch08_crates_modules::{GRAVITY_ACCEL, ROOM_TEMPERATURE_C, section};

// 常量：编译期内联，每次使用都重新「拷贝」进代码
const MAX_RETRIES: u32 = 3;
const GREETING: &str = "Hello, Rust!";

// 静态变量：固定内存地址，整个进程一份
static COUNTER_INIT: i32 = 0;

// ❌ 可变 static 是 unsafe —— safe 代码完全不能用
// static mut PACKETS_SERVED: usize = 0;
// fn main() { PACKETS_SERVED += 1; } // 编译错误

fn main() {
    section("const：编译期常量，每次使用都被内联");
    println!("  MAX_RETRIES = {MAX_RETRIES}");
    println!("  GREETING = {GREETING}");
    println!("  ROOM_TEMPERATURE_C = {ROOM_TEMPERATURE_C} (来自 lib.rs)");

    section("static：进程级单例");
    println!("  GRAVITY_ACCEL = {GRAVITY_ACCEL} m/s² (来自 lib.rs)");
    println!("  COUNTER_INIT = {COUNTER_INIT}");

    section("const vs static 的内存差异");
    println!("  const  → 每次使用被内联进代码（无固定地址）");
    println!("  static → 一份内存，所有引用指向同一地址");

    section("const 可以有「关联常量」(impl 块里)");
    println!("  Vec::<i32>::MAX_LEN = {:?}", size_of_example());

    section("替代可变全局状态：AtomicUsize");
    use std::sync::atomic::{AtomicUsize, Ordering};
    static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(0);
    let old = GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("  fetch_add(1) → 旧值 = {old}");
    let old = GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("  fetch_add(1) → 旧值 = {old}");
    println!("  最终值 = {}", GLOBAL_COUNTER.load(Ordering::SeqCst));
}

/// 演示关联常量。
struct Vec;
impl Vec {
    /// 一个简单的关联常量。
    const MAX_LEN: usize = usize::MAX;
}

fn size_of_example() -> usize {
    std::mem::size_of::<i32>()
}
