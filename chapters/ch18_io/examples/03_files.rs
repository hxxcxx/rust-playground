//! 18.3 文件 I/O：File / OpenOptions
//!
//! 关键结论：
//! - `std::fs::File`：表示一个打开的文件，实现了 Read + Write + Seek。
//! - `File::open(path)` —— 只读打开（失败返回 io::Error）。
//! - `File::create(path)` —— 创建并只写打开（覆盖已存在）。
//! - `OpenOptions`：精细控制 —— read/write/append/create/create_new/truncate。
//! - `fs::read(path)` / `fs::read_to_string(path)` / `fs::write(path, data)`：便捷函数。
//! - 错误处理：所有文件操作返回 io::Result<T>。
//!
//! 注意：本示例会在临时目录创建文件（运行后可清理）。
//!
//! 运行：`cargo run -p ch18_io --example 03_files`

use ch18_io::section;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

fn main() {
    // 用系统临时目录，避免污染项目。
    let dir = std::env::temp_dir().join("ch18_io_demo");
    fs::create_dir_all(&dir).unwrap();
    let path: PathBuf = dir.join("hello.txt");
    println!("  工作目录: {}", dir.display());

    section("File::create + write：创建并写入");
    {
        let mut f = File::create(&path).unwrap();
        f.write_all(b"first line\n").unwrap();
        f.write_all(b"second line\n").unwrap();
    } // 离开作用域自动关闭（Drop）。
    println!("  已写入 hello.txt");

    section("File::open + read：读取");
    {
        let mut f = File::open(&path).unwrap();
        let mut content = String::new();
        f.read_to_string(&mut content).unwrap();
        println!("  内容:\n{}", content.trim_end());
    }

    section("便捷函数：fs::read_to_string");
    let content = fs::read_to_string(&path).unwrap();
    println!("  read_to_string: {} 字节", content.len());

    section("便捷函数：fs::write");
    fs::write(&path, "覆盖写\n").unwrap();
    println!("  fs::write 后内容: {:?}", fs::read_to_string(&path).unwrap());

    section("OpenOptions：追加模式");
    // append = true：写到文件末尾，不覆盖。
    {
        let mut f = OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap();
        writeln!(f, "追加的一行").unwrap();
    }
    println!("  追加后:");
    println!("    {}", fs::read_to_string(&path).unwrap().replace('\n', "\n    "));

    section("OpenOptions：完整控制（读+写+创建+不存在才创建）");
    let path2 = dir.join("config.txt");
    {
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true) // 不存在则创建
            .truncate(true) // 存在则清空
            .open(&path2)
            .unwrap();
        writeln!(f, "config = loaded").unwrap();
    }
    println!("  config.txt: {:?}", fs::read_to_string(&path2).unwrap());

    section("create_new：文件已存在则失败");
    let result = OpenOptions::new()
        .write(true)
        .create_new(true) // 已存在 → 报错
        .open(&path); // path 已存在
    match result {
        Ok(_) => println!("  创建成功（意外）"),
        Err(e) => println!("  预期失败: {e}（文件已存在）"),
    }

    section("文件元数据 metadata");
    let meta = fs::metadata(&path).unwrap();
    println!("  长度: {} 字节", meta.len());
    println!("  是文件? {}", meta.is_file());
    println!("  是目录? {}", meta.is_dir());

    section("目录操作：创建/列出/删除");
    let subdir = dir.join("subdir");
    fs::create_dir(&subdir).unwrap();
    fs::create_dir_all(dir.join("a/b/c")).unwrap();
    // 列出目录内容。
    println!("  {} 下的文件:", dir.display());
    for entry in fs::read_dir(&dir).unwrap() {
        let entry = entry.unwrap();
        println!("    {}", entry.file_name().to_string_lossy());
    }

    section("错误处理：打开不存在的文件");
    let result = File::open(dir.join("nonexistent.txt"));
    match &result {
        Ok(_) => println!("  打开成功（意外）"),
        Err(e) => {
            println!("  错误类型: {}", e.kind());
            println!("  错误信息: {e}");
        }
    }
    // 用 ? 的写法（这里手动 match 演示）。
    let _ = result;

    section("清理临时文件");
    fs::remove_dir_all(&dir).unwrap();
    println!("  已删除 {}", dir.display());
}
