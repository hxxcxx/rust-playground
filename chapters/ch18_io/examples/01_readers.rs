//! 18.1 Readers：Read / BufRead trait
//!
//! 关键结论：
//! - `Read` trait：核心方法 `read(&mut self, buf) -> io::Result<usize>`，读字节到 buf。
//! - 便捷方法（有默认实现）：
//!   * `read_to_end(&mut Vec<u8>)` —— 读全部到 Vec
//!   * `read_to_string(&mut String)` —— 读全部成字符串（要求 UTF-8）
//!   * `read_exact(&mut [u8])` —— 精确读 N 字节（不足报错）
//!   * `bytes()` —— 按字节迭代
//!   * `chain(other)` —— 串联两个 reader
//! - `BufRead`：在 Read 基础上提供「按行读」：
//!   * `read_line(&mut String)` —— 读一行
//!   * `lines()` —— 行迭代器（返回 io::Result<String>）
//! - 字节串 `b"..."` / `&[u8]` 实现 Read，可当 reader 用。
//!
//! 运行：`cargo run -p ch18_io --example 01_readers`

use ch18_io::section;
use std::io::{BufRead, BufReader, Read};

fn main() {
    section("用 &[u8] 当 Reader");
    let data: &[u8] = b"hello world";
    let mut reader = data;
    let mut buf = [0u8; 5];
    let n = reader.read(&mut buf).unwrap();
    println!("  read {} 字节: {:?}", n, &buf[..n]);
    // 再读剩下的。
    let mut rest = String::new();
    reader.read_to_string(&mut rest).unwrap();
    println!("  剩余: {rest:?}");

    section("read_to_string：一次读全部成字符串");
    let data = b"line1\nline2\nline3";
    let mut s = String::new();
    // data 是 &[u8; N]，&data 当 &dyn Read 用。
    (&data[..]).read_to_string(&mut s).unwrap();
    println!("  {s:?}");

    section("read_exact：精确读 N 字节");
    let data = b"abcdef";
    let mut buf = [0u8; 3];
    (&data[..]).read_exact(&mut buf).unwrap();
    println!("  read_exact(3): {:?}", buf);

    section("bytes()：按字节迭代（返回 Result<u8>）");
    // bytes() 来自 Read trait，每项是 io::Result<u8>（可能在读中途出错）。
    for (i, b) in (&b"ABC"[..]).bytes().enumerate() {
        let b = b.unwrap();
        println!("    字节 {i}: {} (0x{b:02X})", b as char);
    }

    section("BufRead：read_line 按行读");
    let data = b"first line\nsecond line\nthird line";
    let mut reader = BufReader::new(&data[..]);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    println!("  第 1 行: {:?}", line.trim_end());
    line.clear();
    reader.read_line(&mut line).unwrap();
    println!("  第 2 行: {:?}", line.trim_end());

    section("lines()：行迭代器");
    let data = "apple\nbanana\ncherry";
    for (i, line) in data.lines().enumerate() {
        println!("    行 {}: {line}", i + 1);
    }
    // 用 BufReader 的 lines()（返回 Result）。
    let reader = BufReader::new(data.as_bytes());
    for line in reader.lines() {
        let line = line.unwrap();
        println!("    [BufReader] {}", line);
    }

    section("chain：串联两个 reader");
    let r1 = b"hello ".as_slice();
    let r2 = b"world".as_slice();
    let mut chained = r1.chain(r2);
    let mut all = String::new();
    chained.read_to_string(&mut all).unwrap();
    println!("  chained: {all:?}");

    section("take(n)：限制最多读 n 字节");
    let data = b"1234567890";
    let mut limited = (&data[..]).take(4); // 最多读 4 字节
    let mut s = String::new();
    limited.read_to_string(&mut s).unwrap();
    println!("  take(4): {s:?}");

    section("标准输入 stdin（演示，不实际等待输入）");
    println!("  （生产代码用 io::stdin().lock() 提高性能）");
    println!("  示例：let mut s = String::new(); io::stdin().read_line(&mut s)?;");
}
