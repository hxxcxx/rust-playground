//! 5.3.5 ~ 5.3.6 包含引用的结构体、不同的生命周期参数
//!
//! 关键结论：
//! - 任何「包含引用的类型」都必须显式写出生命周期参数：`struct S<'a> { r: &'a i32 }`。
//! - `S` 的生命周期 `'a` 受两条约束：
//!   ① `r` 引用的对象必须存活至少 `'a` 那么久；
//!   ② 持有 `S` 的位置必须被 `'a` 包围（`S` 不能比它引用的对象活得久）。
//! - 把带生命周期的类型再放进别的结构体，外层也必须接力传递 `'a`。
//! - 如果结构体里多个引用共用同一个 `'a`，会「过度限制」：
//!   较短引用会让整个 `'a` 被截短，影响较长引用的使用。解决办法：给每个引用独立 `'a`、`'b`。
//! - Rust 强制要求显式生命周期的原因：让类型的「借用行为」从签名就能看明白。
//!
//! 运行：`cargo run -p ch05_references --example 05_lifetimes_in_struct`

use ch05_references::{S, TwoRefs, section};

/// 演示：把引用放进结构体 —— 必须给结构体加生命周期参数 `'a`。
/// ❌ 写 `struct S { r: &i32 }` 会被拒绝：error[E0106]: missing lifetime specifier。
fn struct_with_ref_needs_lifetime() {
    let x = 10;
    let s = S { r: &x }; // S<'a>，'a 由 x 的生命周期决定
    assert_eq!(*s.r, 10);
    println!("✅ struct S<'a> {{ r: &'a i32 }}：合法，*s.r = {}", s.r);
}

/// ❌ 反面教材（注释保留）：把对短命 `x` 的引用存到长命 `s` 中会被拒绝。
/// 编译报错：error: `x` does not live long enough
fn dangling_struct_ref_commented() {
    println!("\n❌ 反面教材（见源码注释）：");
    println!("let s;");
    println!("{{");
    println!("    let x = 10;");
    println!("    s = S {{ r: &x }}; // ❌ 'a 必须包围 s 的使用期，又必须 ≤ x 的生命期 —— 矛盾");
    println!("}}");
    println!("assert_eq!(*s.r, 10);");
    // {
    //     let s;
    //     {
    //         let x = 10;
    //         s = S { r: &x };
    //     }
    //     assert_eq!(*s.r, 10);
    // }
}

/// 演示：解析器模式的典型签名 —— 从字节切片构造一个带生命周期的 Record。
/// 即便不看 Record 定义，仅凭签名就能判断：返回的 Record 借用了输入缓冲区。
fn parse_record_pattern() {
    /// 仅作示意：返回值 Record<'i> 借用了 input
    struct Record<'i>(&'i str);

    fn parse_record<'i>(input: &'i [u8]) -> Record<'i> {
        // 假设输入是 ASCII，直接当作 &str
        let s = std::str::from_utf8(input).expect("utf8");
        Record(s)
    }

    let input = b"hello";
    let rec = parse_record(input);
    // rec 的生命周期与 input 绑定：必须 input 仍存活时使用
    assert_eq!(rec.0, "hello");
    println!("✅ parse_record<'i>(&'i [u8]) -> Record<'i>：返回值借用 input");
}

/// 演示：两个引用共用一个生命周期 `'a` 会过度限制。
/// 用 `TwoRefs<'a, 'b>`（两个独立生命周期）可以解除限制。
///
/// ❌ 反面教材（共用 'a）：下列代码会编译失败
/// ```
/// struct S1<'a> { x: &'a i32, y: &'a i32 }
/// let x = 10;
/// let r;
/// {
///     let y = 20;
///     {
///         let s = S1 { x: &x, y: &y };  // 'a 必须同时覆盖 x 和 y
///         r = s.x;                       // 'a 必须 ≥ r 的使用期
///     }                                   // 但 'a 必须 ≤ y 的生命期 → 矛盾！
/// }
/// println!("{}", r);
/// ```
fn single_lifetime_too_tight_commented() {
    println!("\n❌ 反面教材：两个引用共用 'a 会过度限制（见源码注释）");
    println!("两个引用被迫取「较短者」的生命周期，导致较长者也无法长存。");
}

/// ✅ 正例：给两个引用独立的生命周期 `'a`、`'b`。
/// `s.x` 的生命周期（`'a`）不再受 `s.y`（`'b`）影响。
fn two_independent_lifetimes() {
    let x = 10;
    let r;
    {
        let y = 20;
        {
            // TwoRefs<'a, 'b>：x 用 'a，y 用 'b，互不干扰
            let s = TwoRefs { x: &x, y: &y };
            r = s.x; // r 的生命周期绑定到 'a（即 x），与较短的 y 无关
            assert_eq!(*s.y, 20); // y 仍在作用域，可用
            println!("✅ 两引用独立生命周期：内层 *s.y = 20");
        } // y 在此 drop，但 r 只借用 x，不受影响
    }
    assert_eq!(*r, 10);
    println!("✅ 外层 *r = {r}（不受 y 已 drop 的影响）");
}

fn main() {
    section("包含引用的结构体必须带生命周期参数");
    struct_with_ref_needs_lifetime();

    section("悬空结构体引用为何被拒绝");
    dangling_struct_ref_commented();

    section("签名反映借用：解析器模式");
    parse_record_pattern();

    section("共用单一 'a 的过度限制问题");
    single_lifetime_too_tight_commented();

    section("两个独立生命周期 'a 'b 解除限制");
    two_independent_lifetimes();

    section("总结");
    println!("Rust 强制为含引用的类型写生命周期，是为了让类型签名");
    println!("如实反映它的借用来源 —— 调用者只看签名就知道返回值借用谁。");
}
