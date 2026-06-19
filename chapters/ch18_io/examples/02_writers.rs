//! 18.2 Writers：Write trait
//!
//! 关键结论：
//! - `Write` trait：核心方法 `write(&mut self, buf: &[u8]) -> io::Result<usize>`。
//! - 便捷方法：
//!   * `write!(w, "...", ...)` —— 格式化写入（宏，要 use std::fmt::Write 或 io::Write）
//!   * `write_all(&[u8])` —— 写全部（不返回字节数）
//!   * `flush()` —— 刷新缓冲到目标
//!   * `write_fmt(Arguments)` —— 用 format_args! 写入
//! - `String` 实现了 `std::fmt::Write`（不是 io::Write）—— 用 write! 往 String 写。
//! - `Vec<u8>` 实现了 `io::Write` —— 可当作字节缓冲。
//! - `stdout()` / `stderr()` 是 io::Write。
//! - 缓冲：直接 write 频繁系统调用很慢 → 用 BufWriter 包装。
//!
//! 运行：`cargo run -p ch18_io --example 02_writers`

use ch18_io::section;
use std::fmt::Write as _;
use std::io::{self, Write};

fn main() {
    section("Vec<u8> 当 Writer（io::Write）");
    let mut buf: Vec<u8> = Vec::new();
    buf.write_all(b"hello").unwrap();
    buf.write_all(b" world").unwrap();
    println!("  Vec<u8>: {:?}", buf);
    println!("  转字符串: {:?}", String::from_utf8(buf).unwrap());

    section("write! 往 String 写（fmt::Write）");
    let mut s = String::new();
    // 注意：String 实现的是 std::fmt::Write，不是 io::Write。
    // write! 宏会根据 target 类型选择对应的 Write。
    let _ = write!(s, "{} + {} = {}", 2, 3, 5);
    let _ = write!(s, " (再次)");
    println!("  String: {s:?}");

    section("write! 往 io::Write（Vec<u8>/stdout）写");
    let mut buf: Vec<u8> = Vec::new();
    // 这里 write! 走 io::Write（返回 io::Result）。
    write!(buf, "name=Alice, age={}", 30).unwrap();
    println!("  Vec<u8>: {:?}", String::from_utf8(buf).unwrap());

    section("write_all：写字节切片");
    let mut buf: Vec<u8> = Vec::new();
    buf.write_all(&[0x41, 0x42, 0x43]).unwrap(); // ABC
    println!("  write_all([0x41,0x42,0x43]): {:?}", buf);

    section("stdout / stderr：标准输出");
    // stdout() 返回的 handle 实现了 io::Write。
    let stdout = io::stdout();
    let mut handle = stdout.lock(); // lock 提高性能（避免每次锁）。
    writeln!(handle, "  通过 stdout 写入").unwrap();
    writeln!(handle, "  第二行").unwrap();

    section("flush：刷新缓冲");
    let mut buf: Vec<u8> = Vec::new();
    buf.write_all(b"data").unwrap();
    buf.flush().unwrap(); // Vec 的 flush 是空操作（无缓冲层）
    println!("  flush 后: {:?}", buf);

    section("writeln!：带换行");
    let mut s = String::new();
    let _ = writeln!(s, "第一行");
    let _ = writeln!(s, "第二行");
    println!("  writeln! 产物: {s:?}");

    section("by_ref：借用 &mut self 当 Writer");
    let mut buf: Vec<u8> = Vec::new();
    // by_ref 让你在不转移所有权的情况下用 writer。
    write!(buf.by_ref(), "borrowed").unwrap();
    println!("  by_ref 后: {:?}", buf);

    section("String 的 fmt::Write vs io::Write 区别");
    // String 只实现 fmt::Write（永不失败）。
    // 文件/stdout 实现 io::Write（可能失败，返回 io::Result）。
    // write! 宏对两者都能用，但返回类型不同。
    let mut s = String::new();
    let r1: Result<(), std::fmt::Error> = write!(s, "fmt");
    println!("  write!(String,...) 返回 fmt::Result: {:?}", r1);

    let mut v: Vec<u8> = Vec::new();
    let r2: io::Result<()> = write!(v, "io");
    println!("  write!(Vec<u8>,...) 返回 io::Result: {:?}", r2);
}
