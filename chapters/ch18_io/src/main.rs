//! 第18章 输入与输出 —— 入口。
//!
//! 章节示例：
//! - `01_readers`     —— Read/BufRead：read_to_string/read_line/lines
//! - `02_writers`     —— Write：write!/write_all/flush + BufWriter
//! - `03_files`       —— 文件 I/O：File/OpenOptions/读写/追加
//! - `04_buffered`    —— 缓冲 I/O：BufReader/BufWriter 性能对比
//! - `05_cursor`      —— Cursor：内存中读写 + Seek + 二进制
//! - `06_custom_io`   —— 自定义 Reader/Writer 装饰器 + io::Result 错误处理

fn main() {
    println!("第18章 输入与输出");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch18_io --example 01_readers");
    println!("  cargo run -p ch18_io --example 02_writers");
    println!("  cargo run -p ch18_io --example 03_files");
    println!("  cargo run -p ch18_io --example 04_buffered");
    println!("  cargo run -p ch18_io --example 05_cursor");
    println!("  cargo run -p ch18_io --example 06_custom_io");
}
