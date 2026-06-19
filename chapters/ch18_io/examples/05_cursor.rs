//! 18.5 Cursor：内存中读写 + Seek + 二进制
//!
//! 关键结论：
//! - `Cursor<T>`：把一个 `Vec<u8>` / `&[u8]` 包装成实现了 Read+Write+Seek 的类型。
//! - 内部维护一个「位置指针」，read/write 从当前位置开始。
//! - 用途：
//!   * 解析二进制数据（内存中向前读）。
//!   * 把数据先写到内存再整体输出（构造缓冲）。
//!   * 测试：用 Cursor 代替真实文件做单元测试。
//! - Seek：移动位置指针（SeekFrom::Start/End/Current）。
//!
//! 运行：`cargo run -p ch18_io --example 05_cursor`
//
// 本示例演示基础 read()（返回读到的字节数），clippy 会建议用 read_exact；
// 教学目的保留 read，整体关闭该 lint。
#![allow(clippy::unused_io_amount)]

use ch18_io::section;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

fn main() {
    section("Cursor：内存中的 Reader");
    let data = b"hello world";
    let mut cursor = Cursor::new(data);
    let mut buf = [0u8; 5];
    cursor.read(&mut buf).unwrap();
    println!("  read 5 字节: {:?}", std::str::from_utf8(&buf).unwrap());
    // 再读 5 字节（从位置 5 继续）。
    cursor.read(&mut buf).unwrap();
    println!("  再读 5 字节: {:?}", std::str::from_utf8(&buf).unwrap());

    section("Cursor：内存中的 Writer（写到 Vec<u8>）");
    let mut cursor = Cursor::new(Vec::<u8>::new());
    cursor.write_all(b"hello").unwrap();
    cursor.write_all(b" world").unwrap();
    let into_vec: Vec<u8> = cursor.into_inner();
    println!("  写入后: {:?}", String::from_utf8(into_vec).unwrap());

    section("Seek：移动读写位置");
    let mut cursor = Cursor::new(b"abcdefghij".to_vec());
    // 跳到位置 5。
    cursor.seek(SeekFrom::Start(5)).unwrap();
    let mut buf = [0u8; 3];
    cursor.read(&mut buf).unwrap();
    println!("  seek(5) 后 read 3: {:?}", std::str::from_utf8(&buf).unwrap()); // fgh

    // 相对当前位置移动。
    cursor.seek(SeekFrom::Current(-3)).unwrap();
    cursor.read(&mut buf).unwrap();
    println!("  回退 3 后 read: {:?}", std::str::from_utf8(&buf).unwrap());

    // 从末尾移动。
    cursor.seek(SeekFrom::End(-2)).unwrap();
    cursor.read(&mut buf).unwrap();
    let n = cursor.position();
    println!("  End(-2) 后读到: {:?}", std::str::from_utf8(&buf[..2]).unwrap());

    section("position()：查询当前位置");
    let mut cursor = Cursor::new(b"0123456789");
    cursor.read(&mut [0u8; 3]).unwrap();
    println!("  读 3 字节后 position = {}", cursor.position());

    section("解析二进制：长度前缀 + 字符串 + 小端 i32");
    // 一个二进制序列：[len(1字节)][字符串字节][i32(4字节小端)]。
    // 0x39 0x30 = 12345 的小端低两字节（12345 = 0x3039）。
    let mut bytes = Cursor::new(&[3, b'a', b'b', b'c', 0x39, 0x30, 0, 0][..]);
    // 读 1 字节长度。
    let mut len_buf = [0u8; 1];
    bytes.read_exact(&mut len_buf).unwrap();
    let len = len_buf[0] as usize;
    let mut str_buf = vec![0u8; len];
    bytes.read_exact(&mut str_buf).unwrap();
    let mut int_buf = [0u8; 4];
    bytes.read_exact(&mut int_buf).unwrap();
    let n = i32::from_le_bytes(int_buf);
    println!("  len={len}, str={:?}, i32={n}", String::from_utf8_lossy(&str_buf));

    section("Cursor 用于测试（替代真实文件）");
    // 单元测试里用 Cursor 模拟文件，避免真实 I/O。
    fn count_lines<R: Read>(reader: R) -> usize {
        use std::io::BufRead;
        BufReader::new(reader).lines().map_while(Result::ok).count()
    }
    use std::io::BufReader;
    let cursor = Cursor::new(b"a\nb\nc\n");
    let n = count_lines(cursor);
    println!("  count_lines(Cursor): {n} 行");

    section("大端/小端字节序读写");
    let mut cursor = Cursor::new(Vec::<u8>::new());
    // 写一个 u32 小端。
    cursor.write_all(&42u32.to_le_bytes()).unwrap();
    // 写一个 u32 大端。
    cursor.write_all(&42u32.to_be_bytes()).unwrap();
    let bytes = cursor.into_inner();
    println!("  to_le_bytes(42) = {:?}", &bytes[..4]);
    println!("  to_be_bytes(42) = {:?}", &bytes[4..]);
}
