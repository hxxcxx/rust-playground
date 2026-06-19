//! 17.2 UTF-8 编码：bytes / chars / graphemes
//!
//! 关键结论：
//! - Rust 的 String/&str 是「UTF-8 字节序列」，强制合法 UTF-8。
//! - 一个 char（Unicode 标量值）可能占 1~4 字节：
//!   * ASCII（a-z, 0-9）：1 字节
//!   * 拉丁扩展（é, ñ）：2 字节
//!   * 中文/日文/韩文：3 字节
//!   * emoji（部分）：4 字节
//! - `len()` 返回「字节数」，不是字符数！
//! - `chars().count()` 才是字符数。
//! - 字节下标必须落在「字符边界」上，否则切片会 panic。
//! - graphemes（字形簇）：一个「视觉字符」可能由多个 char 组成（如 é = e + ́），
//!   需要 unicode-segmentation 等外部 crate 处理。
//!
//! 运行：`cargo run -p ch17_strings_text --example 02_utf8_bytes`

use ch17_strings_text::section;

fn main() {
    section("len() 是字节数，不是字符数");
    let ascii = "hello";
    let chinese = "你好";
    let emoji = "😀";
    println!("  \"hello\"  字节数 = {}, 字符数 = {}", ascii.len(), ascii.chars().count());
    println!("  \"你好\"  字节数 = {}, 字符数 = {}", chinese.len(), chinese.chars().count());
    println!("  \"😀\"   字节数 = {}, 字符数 = {}", emoji.len(), emoji.chars().count());

    section("bytes()：按字节迭代（UTF-8 编码）");
    for (i, b) in "AB".bytes().enumerate() {
        println!("    字节 {i}: {b} (0x{b:02X})");
    }
    for b in "中".bytes() {
        println!("    '中' 的字节: 0x{b:02X}");
    }

    section("chars()：按 Unicode 标量值迭代");
    for c in "café".chars() {
        println!("    char: {c} (U+{:04X})", c as u32);
    }
    for c in "你好rust".chars() {
        println!("    char: {c}");
    }

    section("char_indices()：字符 + 它的字节偏移");
    for (byte_offset, ch) in "你好".char_indices() {
        println!("    字节 {byte_offset}: '{ch}'");
    }

    section("字符边界：字节下标必须落在边界");
    let s = "你好";
    // 中文字符每个占 3 字节，所以 0、3 是合法边界。
    println!("  &s[0..3] = {:?}", &s[0..3]); // "你"
    println!("  &s[3..6] = {:?}", &s[3..6]); // "好"
    // 下面会 panic（落在字符中间）：
    // println!("{}", &s[1..4]); // byte index 1 is not a char boundary

    section("安全切分：用 char_indices 找边界");
    let s = "hello世界";
    let prefix = safe_take_chars(s, 5); // 取前 5 个字符
    println!("  前 5 个字符: {prefix:?}");

    section("组合字符：é 可能是 1 个或 2 个 char");
    let composed = "é"; // 单个 char（U+00E9）
    let decomposed = "e\u{0301}"; // e + 组合重音（2 个 char）
    println!("  composed 字符数 = {}", composed.chars().count()); // 1
    println!("  decomposed 字符数 = {}", decomposed.chars().count()); // 2
    println!("  两者视觉相同，但字节不同: {} == {} ? {}", composed.len(), decomposed.len(), composed == decomposed);
    println!("  （字形簇 grapheme 才能正确计数，需外部 crate）");

    section("encode_utf8：char → 字节");
    let mut buf = [0u8; 4];
    let bytes = '中'.encode_utf8(&mut buf);
    println!("  '中'.encode_utf8 → {bytes:?} ({} 字节)", bytes.len());
}

/// 安全地取字符串前 n 个「字符」（按 char，不按字节）。
fn safe_take_chars(s: &str, n: usize) -> &str {
    s.char_indices()
        .nth(n)
        .map(|(idx, _)| &s[..idx])
        .unwrap_or(s)
}
