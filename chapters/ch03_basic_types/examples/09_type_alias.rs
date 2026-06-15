//! 3.9 类型别名：`type` 关键字（类似 C++ 的 typedef）
//!
//! 运行：`cargo run -p ch03_basic_types --example 09_type_alias`

use ch03_basic_types::section;

/// 给现有类型起一个更具语义的名字。
type Bytes = Vec<u8>;

/// 二维尺寸（用元组别名替代新建结构体）。
type Size = (usize, usize);

fn main() {
    section("type 别名：仅是另一个名字，没有新类型");
    let mut buf: Bytes = vec![0; 4];
    fill(&mut buf);
    println!("buf = {buf:?}");
    println!(
        "Bytes 完全等同于 Vec<u8>: {}",
        size_of::<Bytes>() == size_of::<Vec<u8>>()
    );

    section("用作函数签名");
    let s: Size = (800, 600);
    println!("width = {}, height = {}", s.0, s.1);

    section("标准库中的别名：Result 的别名");
    // 标准库大量用 type 别名简化 Result 写法，例如 std::io::Result
    match read_first_line("Cargo.toml") {
        Ok(line) => println!("Cargo.toml 第一行: {line}"),
        Err(e) => println!("读取失败: {e}"),
    }

    section("注意：type 不创建新类型（无类型安全）");
    // 下面两者可以互换 —— 别名仅是文档性，不防误用
    let a: Bytes = vec![1, 2, 3];
    let b: Vec<u8> = a; // 类型完全相同
    println!("{b:?}");
}

fn fill(buf: &mut [u8]) {
    for (i, byte) in buf.iter_mut().enumerate() {
        *byte = i as u8;
    }
}

/// std::io::Result<T> 就是 type Result<T, std::io::Error> 的别名
fn read_first_line(path: &str) -> std::io::Result<String> {
    use std::io::{BufRead, BufReader};
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().next().unwrap()?)
}
