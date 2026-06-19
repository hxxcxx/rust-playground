//! 17.3 字符串切片 / 字节字符串 / 安全切分
//!
//! 关键结论：
//! - 切片 `&s[a..b]` 按「字节范围」，必须落在字符边界，否则 panic。
//! - 字节字符串 `b"..."`：类型是 `&[u8; N]`，不是 &str —— 用于二进制数据。
//! - 原始字节 `Vec<u8>`：不是字符串；想转 &str 用 `str::from_utf8`（可能失败）。
//! - 想按「字符」切片 → 用 chars().take(n).collect 或 char_indices。
//! - `split_at(byte_idx)`：在字节处一分为二（也要求边界合法）。
//!
//! 运行：`cargo run -p ch17_strings_text --example 03_slicing`

use ch17_strings_text::section;

fn main() {
    section("切片：按字节范围，要求字符边界");
    let s = "Hello, 世界!";
    // ASCII 部分可以任意切。
    println!("  &s[0..5] = {:?}", &s[0..5]); // "Hello"
    println!("  &s[7..10] = {:?}", &s[7..10]); // "世"（3 字节）

    section("split_at：按字节分裂");
    let (left, right) = "hello world".split_at(5);
    println!("  split_at(5): {left:?} | {right:?}");

    section("按字符切片（安全）");
    let s = "你好世界";
    let first_two: String = s.chars().take(2).collect();
    let rest: String = s.chars().skip(2).collect();
    println!("  前 2 字符: {first_two:?}, 剩余: {rest:?}");

    section("字节字符串 b\"...\"");
    let bytes: &[u8; 5] = b"hello";
    println!("  b\"hello\" 类型是 &[u8; N]: {bytes:?}");
    // 可以当字节切片用。
    let slice: &[u8] = &bytes[..3];
    println!("  &bytes[..3] = {slice:?}");
    // 转成 &str（ASCII 安全）。
    let as_str = std::str::from_utf8(bytes).unwrap();
    println!("  from_utf8 → {as_str:?}");

    section("字节字符串里的转义");
    // b"..." 里可以用 \xHH（任意字节）、\n \t 等。
    let raw: &[u8; 2] = b"A\x41"; // A 和 0x41(也是 A)
    println!("  b\"A\\x41\" = {raw:?}");

    section("Vec<u8> 与 &str 的转换");
    let bytes: Vec<u8> = vec![72, 73]; // 'H', 'I'
    // from_utf8 可能失败（非法 UTF-8）。
    match std::str::from_utf8(&bytes) {
        Ok(s) => println!("  Vec<u8> → &str: {s:?}"),
        Err(e) => println!("  非法 UTF-8: {e}"),
    }
    // 非法字节会失败。
    let bad = vec![0xFF, 0xFE];
    let result = std::str::from_utf8(&bad);
    println!("  非法字节 from_utf8 = {:?}", result.map(|_| "ok"));

    section("String 内部：Vec<u8> 的 UTF-8 包装");
    let s = String::from("hi");
    // as_bytes 借用内部字节（零拷贝）。
    let bytes: &[u8] = s.as_bytes();
    println!("  String.as_bytes() = {bytes:?}");
    // into_bytes 拿走所有权得到 Vec<u8>。
    let owned_bytes: Vec<u8> = s.into_bytes();
    println!("  String.into_bytes() = {owned_bytes:?}");

    section("r 原始字符串：忽略转义");
    let raw = r"C:\Users\name\path"; // 反斜杠无需转义
    println!("  r\"...\": {raw}");
    let with_quotes = r#"含 "引号" 的字符串"#;
    println!("  r#\"...\"#: {with_quotes}");

    section("字节切片的 is_ascii / is_utf8 判断");
    println!("  \"hello\".is_ascii() = {}", "hello".is_ascii());
    println!("  \"你好\".is_ascii() = {}", "你好".is_ascii());
}
