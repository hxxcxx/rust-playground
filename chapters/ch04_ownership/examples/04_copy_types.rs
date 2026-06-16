//! 4.3 Copy 类型：移动的例外
//!
//! 关键结论：
//! - 对 `Copy` 类型，赋值/传参/返回是「按位复制」，源变量仍可用、值不变。
//! - 标准 `Copy` 类型：所有机器整数/浮点类型、`char`、`bool`；以及由 `Copy` 类型组成的元组/定长数组。
//! - 不是 `Copy` 的类型：`String`（拥有堆缓冲区）、`Box<T>`、`Vec<T>`、`File`、`MutexGuard` 等
//!   —— 凡是 drop 时需要做特殊动作（释放资源）的类型都不能是 `Copy`。
//! - 用户自定义结构体/枚举「默认不是 Copy」；若所有字段都是 Copy，可用 `#[derive(Copy, Clone)]` 声明。
//!
//! 运行：`cargo run -p ch04_ownership --example 04_copy_types`

use ch04_ownership::{section, Label};

/// 演示：`String`（非 Copy）会移动；`i32`（Copy）会复制。
fn string_vs_i32() {
    // String：移动
    let string1 = "somnambulance".to_string();
    let string2 = string1; // string1 被 move，现在未初始化
    // println!("{string1}"); // ❌ borrow of moved value
    println!("string2 = {string2}");

    // i32：复制
    let num1: i32 = 36;
    let num2 = num1; // num1 被「按位复制」，仍然可用
    println!("num1 = {num1}, num2 = {num2}（两者相互独立）");
}

/// 演示：把值传给函数 —— Copy 类型传完后调用方仍可用。
fn print_label(l: Label) {
    println!("STAMP: {}", l.number);
} // 对于 Copy 类型，函数结束时只是丢弃栈上的副本，原值仍归调用者

/// 演示：不实现 Copy 的结构体传给函数会被 move 走。
#[derive(Debug)]
struct NonCopy {
    text: String, // String 非 Copy，所以 NonCopy 也无法 Copy
}

fn consume_non_copy(_nc: NonCopy) {} // 接管所有权

/// 演示：标准 Copy 类型 —— 整数、浮点、char、bool、由 Copy 组成的元组/数组。
fn standard_copy_types() {
    // bool / char
    let b = true;
    let b2 = b;
    println!("bool: b={b}, b2={b2}");

    let c = '🦀';
    let c2 = c;
    println!("char: c={c}, c2={c2}");

    // 浮点
    let f = 1.5_f64;
    let f2 = f;
    println!("f64: f={f}, f2={f2}");

    // 由 Copy 组成的元组也是 Copy
    let pair = (1_i32, 2_i32);
    let pair2 = pair;
    println!("tuple: pair={pair:?}, pair2={pair2:?}");

    // 定长数组（[T; N]）在 T 是 Copy 时也是 Copy
    let arr = [0_u8; 4];
    let arr2 = arr;
    println!("array: arr={arr:?}, arr2={arr2:?}");
}

/// 演示：仅含 Copy 字段的结构体，可以 `#[derive(Copy, Clone)]`，详见 `ch04_ownership::Label`。
/// 它定义在 lib.rs：
///   #[derive(Copy, Clone)]
///   pub struct Label { pub number: u32 }
fn user_defined_copy_type() {
    let l = Label { number: 3 };
    print_label(l); // Copy：传参只是复制
    println!("My label number is: {}", l.number); // l 仍然可用
}

// 演示：尝试给「字段不全是 Copy」的结构体加 `#[derive(Copy, Clone)]` 会编译失败。
// 解开下面的注释会看到：
//   error[E0204]: the trait `Copy` may not be implemented for this type
//   ---- this field does not implement `Copy`
// #[derive(Copy, Clone)]
// struct StringLabel {
//     name: String, // ← String 不是 Copy
// }

fn main() {
    section("String 移动 vs i32 复制");
    string_vs_i32();

    section("非 Copy 类型传给函数会被 move 走");
    let nc = NonCopy {
        text: "hello".to_string(),
    };
    consume_non_copy(nc);
    // println!("{:?}", nc); // ❌ borrow of moved value

    section("标准 Copy 类型：bool / char / f64 / 元组 / 定长数组");
    standard_copy_types();

    section("用户自定义 Copy 类型（#[derive(Copy, Clone)]）");
    user_defined_copy_type();

    section("为什么默认不自动 Copy？");
    println!("一旦类型设为 Copy，将来想加堆字段/资源就会改动大量调用点。");
    println!("所以语言把决定权交给设计者：需要显式 #[derive(Copy, Clone)]。");
}
