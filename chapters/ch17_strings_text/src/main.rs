//! 第17章 字符串和文本 —— 入口。
//!
//! 章节示例：
//! - `01_string_str`   —— String vs &str、互转、所有权
//! - `02_utf8_bytes`   —— UTF-8 编码、bytes/chars/graphemes、字节下标
//! - `03_slicing`      —— 字符串切片、字节字符串 b"..."、安全切分
//! - `04_building`     —— 构建字符串：push/extend/+/write!/format!
//! - `05_formatting`   —— 格式化：{}/{:?}/{:#?}/对齐/精度/进制/Display
//! - `06_parse_convert`—— 解析 parse / ToString / FromStr / &str 与 String 转换

fn main() {
    println!("第17章 字符串和文本");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch17_strings_text --example 01_string_str");
    println!("  cargo run -p ch17_strings_text --example 02_utf8_bytes");
    println!("  cargo run -p ch17_strings_text --example 03_slicing");
    println!("  cargo run -p ch17_strings_text --example 04_building");
    println!("  cargo run -p ch17_strings_text --example 05_formatting");
    println!("  cargo run -p ch17_strings_text --example 06_parse_convert");
}
