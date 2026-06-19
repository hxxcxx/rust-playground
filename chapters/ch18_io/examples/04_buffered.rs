//! 18.4 缓冲 I/O：BufReader / BufWriter
//!
//! 关键结论：
//! - 每次裸 read/write 都是一次「系统调用」，开销大。
//! - BufReader/BufWriter：在内存维护一个缓冲区（默认 8KB），批量读写。
//!   * BufReader：一次 read 系统调用读满缓冲，后续 read 从缓冲取。
//!   * BufWriter：写到缓冲，满了才一次性 flush 到底层（减少系统调用）。
//! - 适合：频繁小量读写（如逐行读/逐字段写）。
//! - 不适合：已经是大块读写（缓冲反而多余）。
//! - 必须 flush/drop 才保证数据真正写入！
//!
//! 运行：`cargo run -p ch18_io --example 04_buffered`

use ch18_io::section;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::time::Instant;

fn main() {
    section("BufReader：逐行读文件");
    let dir = std::env::temp_dir().join("ch18_io_buf");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("lines.txt");

    // 先造一个大文件。
    {
        let f = File::create(&path).unwrap();
        let mut w = BufWriter::new(f);
        for i in 0..1000 {
            writeln!(w, "这是第 {i} 行").unwrap();
        }
        w.flush().unwrap(); // 重要：确保缓冲写入文件！
    }
    println!("  已生成 1000 行的文件");

    // 用 BufReader 逐行读。
    {
        let f = File::open(&path).unwrap();
        let reader = BufReader::new(f);
        let mut count = 0;
        for line in reader.lines() {
            let _line = line.unwrap();
            count += 1;
        }
        println!("  BufReader 读到 {count} 行");
    }
    // 注意：lines() 来自 BufRead trait（已在顶部 use）。

    section("性能对比：无缓冲 vs BufWriter");
    let n = 10_000;
    let unbuf_path = dir.join("unbuf.txt");
    let buf_path = dir.join("buf.txt");

    // 无缓冲：每次 write 都系统调用。
    let t = Instant::now();
    {
        let mut f = File::create(&unbuf_path).unwrap();
        for i in 0..n {
            writeln!(f, "{i}").unwrap();
        }
    }
    let unbuf_time = t.elapsed();

    // 有缓冲：批量写。
    let t = Instant::now();
    {
        let f = File::create(&buf_path).unwrap();
        let mut w = BufWriter::new(f);
        for i in 0..n {
            writeln!(w, "{i}").unwrap();
        }
        w.flush().unwrap();
    }
    let buf_time = t.elapsed();

    println!("  无缓冲 ×{n}: {unbuf_time:?}");
    println!("  BufWriter ×{n}: {buf_time:?}");
    println!("  （BufWriter 通常快很多）");

    section("自定义缓冲区大小");
    // with_capacity 指定缓冲大小（字节数）。
    let f = File::create(dir.join("big_buf.txt")).unwrap();
    let _writer = BufWriter::with_capacity(64 * 1024, f); // 64KB 缓冲
    println!("  用 with_capacity 设 64KB 缓冲");

    section("BufReader 性能对比：逐字节读");
    // 准备数据。
    std::fs::write(&path, vec![b'A'; 100_000]).unwrap();

    // 无缓冲逐字节读。
    let t = Instant::now();
    {
        let mut f = File::open(&path).unwrap();
        let mut byte = [0u8; 1];
        let mut count = 0;
        while f.read(&mut byte).unwrap() > 0 {
            count += 1;
        }
        let _ = count;
    }
    let unbuf_byte_time = t.elapsed();

    // BufReader 逐字节读（缓冲命中）。
    let t = Instant::now();
    {
        let f = File::open(&path).unwrap();
        let mut reader = BufReader::new(f);
        let mut byte = [0u8; 1];
        let mut count = 0;
        while reader.read(&mut byte).unwrap() > 0 {
            count += 1;
        }
        let _ = count;
    }
    let buf_byte_time = t.elapsed();

    println!("  无缓冲逐字节读 100K: {unbuf_byte_time:?}");
    println!("  BufReader 逐字节读 100K: {buf_byte_time:?}");
    println!("  （逐字节读时 BufReader 提升巨大）");

    section("忘记 flush 的陷阱");
    // BufWriter 在 Drop 时会尝试 flush，但 Drop 中的错误会被忽略！
    // 重要数据要显式 flush 并检查结果。
    println!("  （重要数据应显式 flush()? 而非依赖 Drop）");

    section("清理");
    std::fs::remove_dir_all(&dir).unwrap();
    println!("  已删除临时目录");
}

// 引入 BufRead 的 lines() 方法（放最后避免顶部混乱）。
// 已在第一个 section 内 use，这里再确认可见。
