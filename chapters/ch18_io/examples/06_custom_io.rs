//! 18.6 自定义 Reader/Writer 装饰器 + io::Result 错误处理
//!
//! 关键结论：
//! - Reader/Writer 是 trait，可以「包装」已有类型实现装饰器模式。
//! - 装饰器：实现 Read/Write，内部持有被包装对象，转发时加额外逻辑。
//! - 典型：日志、计数、加密、压缩、格式转换。
//! - 错误处理：io::Result<T> = Result<T, io::Error>。
//!   * io::Error::new(kind, msg) 构造错误。
//!   * io::ErrorKind 分类（NotFound/PermissionDenied/UnexpectedEof...）。
//!   * ? 运算符在返回 io::Result 的函数里自动传播。
//!
//! 运行：`cargo run -p ch18_io --example 06_custom_io`

use ch18_io::{NumberedWriter, UpperReader, section};
use std::io::{self, Read, Write};

fn main() {
    section("自定义 Writer：NumberedWriter 给每行加行号");
    // 用 Vec<u8> 当底层 writer。
    let underlying = Vec::<u8>::new();
    let mut numbered = NumberedWriter::new(underlying);
    writeln!(numbered, "第一行").unwrap();
    writeln!(numbered, "第二行").unwrap();
    write!(numbered, "第三行（无换行）").unwrap();
    let buf = numbered.into_inner();
    let result = String::from_utf8(buf).unwrap();
    println!("{result}");

    section("自定义 Reader：UpperReader 把读出的字节转大写");
    let data = b"hello world";
    let mut reader = UpperReader::new(&data[..]);
    let mut s = String::new();
    reader.read_to_string(&mut s).unwrap();
    println!("  原始: {:?}", std::str::from_utf8(data).unwrap());
    println!("  转大写后: {s:?}");

    section("组合多个装饰器：UpperReader 包装 NumberedWriter");
    // 写入时：NumberedWriter 加行号 → 转大写。
    // 这里演示「读取」链：把已写入的内容用 UpperReader 读出再转大写。
    let written = {
        let underlying = Vec::<u8>::new();
        let mut numbered = NumberedWriter::new(underlying);
        writeln!(numbered, "hello").unwrap();
        writeln!(numbered, "world").unwrap();
        numbered.into_inner()
    };
    println!("  原始写入: {:?}", String::from_utf8_lossy(&written));
    // 再用 UpperReader 读一遍。
    let mut reader = UpperReader::new(&written[..]);
    let mut uppered = String::new();
    reader.read_to_string(&mut uppered).unwrap();
    println!("  UpperReader 读出: {uppered:?}");

    section("io::Result 与 ? 运算符");
    match do_io_work() {
        Ok(s) => println!("  成功: {s}"),
        Err(e) => println!("  失败: {e}"),
    }

    section("io::Error 与 ErrorKind");
    // 构造一个自定义错误。
    let err = io::Error::new(io::ErrorKind::InvalidInput, "参数非法");
    println!("  自定义错误: {err}");
    println!("  kind: {:?}", err.kind());

    // 匹配不同错误类型。
    let result = std::fs::read_to_string("/nonexistent/path/file.txt");
    match &result {
        Ok(_) => println!("  读到了（意外）"),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => println!("  NotFound: 文件不存在"),
            io::ErrorKind::PermissionDenied => println!("  权限不足"),
            _ => println!("  其它错误: {e}"),
        },
    }

    section("把 io::Error 转成自定义错误（? 配合 From）");
    // 实现 From<io::Error> 后，? 能自动把 io::Error 转成自定义错误类型。
    match read_with_custom_error() {
        Ok(n) => println!("  读到 {n} 字节"),
        Err(AppError::Io(e)) => println!("  IO 错误: {e}"),
        Err(AppError::Empty) => println!("  空数据错误"),
    }

    section("copy：高效的 reader → writer 拷贝");
    let mut reader = b"copy this data".as_slice();
    let mut writer = Vec::<u8>::new();
    let n = io::copy(&mut reader, &mut writer).unwrap();
    println!("  copy 了 {n} 字节: {:?}", String::from_utf8(writer).unwrap());

    section("装饰器的实际用途总结");
    println!("  - 日志/计数（统计读写字节数）");
    println!("  - 加密/压缩（透明地变换数据）");
    println!("  - 格式转换（大小写、编码、行尾）");
    println!("  - 缓冲（BufReader/BufWriter 本身就是装饰器）");
}

/// 演示 ? 在 io::Result 函数中的用法。
fn do_io_work() -> io::Result<String> {
    let data = b"some data";
    let mut reader = &data[..];
    let mut s = String::new();
    // ? 自动传播 io::Error。
    reader.read_to_string(&mut s)?;
    Ok(s)
}

/// 自定义应用错误类型。
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Empty,
}

// 实现 From<io::Error> 让 ? 能自动转换。
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

/// 用自定义错误类型 + ? 读取（演示 From 转换）。
fn read_with_custom_error() -> Result<usize, AppError> {
    let data = b"hello";
    let mut reader = &data[..];
    let mut buf = [0u8; 10];
    // 这里的 ? 把 io::Error 自动转成 AppError（因为实现了 From）。
    let n = reader.read(&mut buf)?;
    if n == 0 {
        return Err(AppError::Empty);
    }
    Ok(n)
}
