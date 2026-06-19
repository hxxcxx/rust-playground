//! 第18章 输入与输出（Input and Output）—— 共享工具与示例。
//!
//! 本章核心：std::io 中的 Reader/Writer 模型，以及文件/终端/缓冲 I/O。
//!
//! - `Read` trait：从数据源「读字节」—— read_to_string / read / read_exact。
//! - `BufRead` trait：在 Read 基础上「按行读」—— lines() / read_line()。
//! - `Write` trait：向目标「写字节」—— write! / write_all / flush。
//! - Reader/Writer 是「字节导向」的：读写字节，要配合字符串需 UTF-8 转换。
//! - 文件 I/O：`File` / `OpenOptions`（读/写/追加/创建）。
//! - 缓冲：`BufReader` / `BufWriter` 减少系统调用次数（默认 8KB）。
//! - 标准流：`stdin()` / `stdout()` / `stderr()`。
//! - `io::Result<T>`：I/O 操作的返回类型（错误 = io::Error）。
//! - `Cursor<T>`：把内存中的 `Vec<u8>`/`&[u8]` 当作 Reader/Writer/Seek。

use std::io::{self, Write};

// =======================================================================
// 工具函数
// =======================================================================

/// 打印带标题的分割线（与前几章风格一致）。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

// =======================================================================
// 示例：实现自定义 Writer（一个「带行号」的 Writer 装饰器）
// =======================================================================

/// 包装一个 Writer，在每行前面加上「行号」。
///
/// 演示「装饰器模式」：实现 Write trait，转发到底层 writer。
pub struct NumberedWriter<W: Write> {
    inner: W,
    line: u32,
    at_line_start: bool,
}

impl<W: Write> NumberedWriter<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            line: 1,
            at_line_start: true,
        }
    }

    /// 消费自身，返回内部的 writer。
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write> Write for NumberedWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut written = 0;
        for &byte in buf {
            if self.at_line_start {
                // 行首：先写行号前缀。
                let prefix = format!("{:>4}: ", self.line);
                self.inner.write_all(prefix.as_bytes())?;
                self.at_line_start = false;
            }
            self.inner.write_all(&[byte])?;
            written += 1;
            if byte == b'\n' {
                self.line += 1;
                self.at_line_start = true;
            }
        }
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

// =======================================================================
// 示例：实现自定义 Reader（一个「把字节转大写」的 Reader 装饰器）
// =======================================================================

/// 包装一个 Reader，读出的字节都转成大写。
///
/// 演示「装饰器模式」：实现 Read trait。
pub struct UpperReader<R: io::Read> {
    inner: R,
}

impl<R: io::Read> UpperReader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner }
    }
}

impl<R: io::Read> io::Read for UpperReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        // 把读到的字节转大写（仅 ASCII）。
        for byte in &mut buf[..n] {
            if *byte >= b'a' && *byte <= b'z' {
                *byte -= 32;
            }
        }
        Ok(n)
    }
}
