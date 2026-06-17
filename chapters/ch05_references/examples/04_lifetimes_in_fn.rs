//! 5.3.2 ~ 5.3.7 函数签名中的生命周期参数、'static、返回引用、省略规则
//!
//! 关键结论：
//! - 函数签名必须如实反映函数体对引用的使用方式。例如要把参数存进静态变量，就必须标 `'static`。
//! - 写出生命周期的形式：`fn f<'a>(p: &'a i32)`，读作「对于任意生命周期 'a」。
//! - 返回引用时，编译器需要知道返回引用与哪个入参的生命周期绑定。
//! - 生命周期「省略规则」（lifetime elision）让常见场景无需手写 `'a`：
//!   规则1：每个引用参数各自分配一个独立生命周期；
//!   规则2：如果只有一个入参生命周期，返回值的生命周期默认取它；
//!   规则3：方法中有 `&self`/`&mut self` 时，返回值默认绑定 `self` 的生命周期。
//!   三条规则都无法确定时，编译器报 E0106，要求显式写出。
//!
//! 运行：`cargo run -p ch05_references --example 04_lifetimes_in_fn`

// 教学示例：刻意写出显式生命周期 / 用可变静态变量演示 'static / 用 for 循环演示算法。
#![allow(
    unsafe_code,
    clippy::needless_lifetimes,
    clippy::manual_find,
    clippy::for_kv_map
)]

use ch05_references::section;

// —— 显式生命周期的几种典型签名 ——

/// 一般函数：`fn g<'a>(p: &'a i32)` —— 不假设参数生命周期有多长，
/// 也不会把参数存到比调用更长寿的地方。
fn g(_p: &i32) {
    // 省略形式等价于 fn g<'a>(_p: &'a i32)
}

/// 函数签名如实反映行为：此函数只接受 `'static` 引用（因为它要存进静态变量）。
/// Rust 会强制要求签名标 `'static`，否则报 E0312「explicit lifetime required」。
static mut STASH: &i32 = &128;
fn store_to_static(p: &'static i32) {
    // SAFETY：教学示例；可变静态变量必须在 unsafe 块中访问
    unsafe {
        STASH = p;
    }
}

/// 返回引用：编译器需要知道返回值与哪个入参绑定。
/// 省略形式 `fn smallest(v: &[i32]) -> &i32` 由规则 2 推导为 `&'a`。
fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

/// 多入参 + 返回引用：如果返回值只来自某一个参数，
/// 给两个参数不同的生命周期 `'a`/`'b` 可以放宽调用者的限制。
fn pick_first<'a, 'b>(_other: &'b i32, first: &'a i32) -> &'a i32 {
    first
}

// —— 演示函数 ——

fn explicit_lifetime_in_signature() {
    let x = 10;
    g(&x); // 调用 g 时无需提及 'a，使用时由编译器推断
    println!("✅ 普通函数 g(&i32)：签名反映它不会把参数存得比调用更久");
}

fn static_lifetime_demonstration() {
    // 只有 'static 引用才能存进静态变量
    static WORTH_POINTING_AT: i32 = 1000;
    store_to_static(&WORTH_POINTING_AT);
    // SAFETY：读取 STASH
    let stored = unsafe { *STASH };
    println!("✅ store_to_static 接收 &'static i32，存入静态变量 = {stored}");

    // 局部变量不能传入（注释掉的代码会编译失败）：
    // let local = 5;
    // store_to_static(&local); // ❌ `local` does not have a static lifetime
    println!("❌ 局部变量的引用不能传给要求 &'static 的函数（见源码注释）");
}

fn return_reference_tied_to_input() {
    let parabola = [9, 4, 1, 0, 1, 4, 9];
    let s = smallest(&parabola);
    // s 必须在 parabola 仍然存活时使用，因为返回值的生命周期绑定到入参
    assert_eq!(*s, 0);
    println!("✅ smallest 返回的引用与入参 &parabola 同生命周期，*s = {s}");

    // ❌ 把 s 用到 parabola 之外会失败：
    // let s;
    // {
    //     let parabola = [9, 4, 1, 0, 1, 4, 9];
    //     s = smallest(&parabola);
    // } // parabola 在此 drop
    // assert_eq!(*s, 0); // ❌ `parabola` does not live long enough
}

fn multiple_lifetimes_losening() {
    let short_lived = 1;
    let result;
    {
        let long_lived = 100;
        // pick_first 用了独立生命周期 'a 'b，所以 result 绑定到 long_lived，
        // 不受 short_lived 的较短生命周期影响
        result = pick_first(&short_lived, &long_lived);
        assert_eq!(*result, 100);
        println!("✅ 多生命周期参数放宽调用限制：*result = {result}");
    }
}

/// 演示：方法中的省略规则 —— 返回值默认绑定 `self`。
struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    /// 省略形式：`fn find_by_prefix(&self, prefix: &str) -> Option<&String>`
    /// 等价于 `fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String>`
    /// —— 规则 3：有 &self 时，返回值默认绑定 self 的生命周期。
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for s in &self.elements {
            if s.starts_with(prefix) {
                return Some(s);
            }
        }
        None
    }
}

fn elision_in_methods() {
    let table = StringTable {
        elements: vec![
            "alpha".to_string(),
            "alphabet".to_string(),
            "beta".to_string(),
        ],
    };
    if let Some(found) = table.find_by_prefix("alph") {
        // found 的生命周期绑定到 &table，所以可在 table 仍存活时使用
        assert_eq!(found, "alpha");
        println!("✅ 方法省略规则：返回引用绑定 &self → 找到 {found:?}");
    }
}

fn main() {
    section("函数签名必须如实反映引用行为");
    explicit_lifetime_in_signature();

    section("存进静态变量必须标 &'static");
    static_lifetime_demonstration();

    section("返回引用：生命周期绑定到入参");
    return_reference_tied_to_input();

    section("多生命周期参数：放宽调用限制");
    multiple_lifetimes_losening();

    section("省略规则③：方法返回值默认绑定 self");
    elision_in_methods();

    section("省略规则小结");
    println!("① 每个引用入参各得一个独立生命周期");
    println!("② 单个入参生命周期时，返回值取它");
    println!("③ 方法有 &self/&mut self 时，返回值取 self 的生命周期");
    println!("三条都无法确定时，编译器报 E0106，要求显式写出。");
}
