//! 9.2 元组结构体 + 类单元结构体 + Newtype 模式
//!
//! 关键结论：
//! - 元组结构体 `struct Name(T1, T2);` 用 `Name(v1, v2)` 构造，`.0 .1` 访问字段。
//! - 类单元结构体 `struct Name;` 没有字段，零字节大小。
//! - Newtype 模式：单字段元组结构体，用于获得「类型层面的不同」（更严格类型检查）。
//! - 选用哪种取决于可读性 / 是否需要字段名 / 是否就是包装一层。
//!
//! 运行：`cargo run -p ch09_structs --example 02_tuple_unit_structs`

use ch09_structs::{Ascii, Bounds, Onesuch, section};

fn main() {
    section("元组结构体");
    let image_bounds = Bounds(1024, 768);
    println!(
        "  Bounds(1024, 768) → .0={} .1={}",
        image_bounds.0, image_bounds.1
    );
    println!("  .0 * .1 = {}", image_bounds.0 * image_bounds.1);

    section("类单元结构体：零字节");
    let _o = Onesuch;
    println!(
        "  size_of::<Onesuch>() = {} 字节",
        std::mem::size_of::<Onesuch>()
    );
    println!("  （没有任何字段，但仍是合法的「类型」）");

    section("Newtype 模式：单字段元组结构体");
    // 比起直接用 Vec<u8>，包装成 Ascii 让类型系统帮我们区分「普通字节」和「ASCII 文本」
    let text = Ascii(vec![b'h', b'i']);
    println!("  Ascii 内部: {:?}", text.0);
    fn process_ascii(_a: &Ascii) {
        // 函数签名要求 Ascii，防止误传普通 Vec<u8>
    }
    process_ascii(&text);
    println!("  process_ascii 成功调用");

    section("三种结构体对比");
    println!("  具名字段  : struct P {{ x: f64, y: f64 }}     字段有名字");
    println!("  元组结构体: struct Bounds(usize, usize);     按位置访问");
    println!("  类单元    : struct Onesuch;                  没有字段");
    println!("  Newtype   : struct Ascii(Vec<u8>);           严格类型检查");
}
