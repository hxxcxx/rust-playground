//! 17.5 格式化：{} / {:?} / 对齐 / 精度 / 进制 / Display
//!
//! 关键结论：
//! - `format!` / `println!` 用「格式化字符串 + 参数」：
//!   * `{}`   —— Display（人类可读，需实现 Display）
//!   * `{:?}` —— Debug（开发者可读，通常 derive）
//!   * `{:#?}`—— Debug 美化（多行缩进）
//! - 格式说明符：`{:[fill][align][width][.precision][type]}`
//!   * 对齐：< 左 / > 右 / ^ 居中（前面加填充字符）
//!   * 宽度：最小字段宽度
//!   * 精度：`.N` 浮点小数位 / 字符串截断
//!   * 进制：b 二进制 / o 八进制 / x 十六进制 / e 科学计数
//! - 位置参数 `{0}` / 命名参数 `{name}`。
//! - 自定义类型：实现 Display 控制 `{}` 输出。
//!
//! 运行：`cargo run -p ch17_strings_text --example 05_formatting`

use ch17_strings_text::{Complex, Matrix, section};

fn main() {
    section("基本：{} (Display) vs {:?} (Debug)");
    let s = "hello";
    let n = 42;
    println!("  {{}}:   {s}, {n}");
    println!("  {{:?}}: {:?}, {:?}", s, n);

    section("{:#?}：Debug 美化（多行缩进）");
    let data = vec![("alice", 30), ("bob", 25)];
    println!("  {data:#?}");

    section("对齐：左 < / 右 > / 居中 ^");
    println!("  [{:<10}]", "left"); // 左对齐，宽 10
    println!("  [{:>10}]", "right"); // 右对齐
    println!("  [{:^10}]", "center"); // 居中

    section("填充字符：在对齐符前加字符");
    println!("  [{:*<10}]", "pad"); // 用 * 填充
    println!("  [{:0>10}]", 42); // 用 0 填充
    println!("  [{:-^10}]", "X"); // 用 - 填充居中

    section("宽度与精度");
    println!("  [{:5}]", 42); // 宽度 5
    // 用一个非数学常数的浮点数演示精度。
    println!("  [{:.3}]", 12.3456); // 3 位小数
    println!("  [{:10.3}]", 12.3456); // 宽 10 + 3 位小数
    println!("  [{:.3}]", "abcdefghij"); // 字符串截到 3 字符

    section("进制：b / o / x / X");
    println!("  二进制:    {:b}", 255);
    println!("  八进制:    {:o}", 255);
    println!("  十六进制:  {:x}", 255);
    println!("  十六大写:  {:X}", 255);
    println!("  带 0x 前缀: {:#x}", 255);

    section("带符号 + 和补零");
    println!("  {{:+}}:  {:+}", 42); // 正数也带 +
    println!("  {{:+}}:  {:+}", -42);
    println!("  {{:08}}: {:08}", 42); // 补零到 8 位
    println!("  {{:+08}}:{:+08}", 42); // 带符号 + 补零

    section("科学计数法 e / E");
    println!("  {{:e}}:  {:e}", 12345.678);
    println!("  {{:E}}:  {:E}", 12345.678);

    section("位置参数与命名参数");
    // 教学：演示位置/命名参数（clippy 会建议把字面量直接写进字符串，这里保留演示）。
    #[allow(clippy::print_literal)]
    {
        println!("  {0} {1} {0}", "A", "B"); // 位置复用
        println!("  {name} 今年 {age} 岁", name = "Alice", age = 30); // 命名
    }

    section("指针地址 {:p}");
    let x = 42;
    println!("  &x 的地址: {:p}", &x);

    section("? 的变体：{:x?} 十六进制 Debug");
    let bytes = [0u8, 1, 255, 16];
    println!("  {{:?}}:  {:?}", bytes);
    println!("  {{:x?}}: {:x?}", bytes);

    section("自定义 Display：复数");
    let c = Complex::new(3.0, -4.0);
    println!("  Display: {c}");
    println!("  Debug:   {c:?}");
    let c2 = Complex::new(1.5, 2.7);
    println!("  {c2}");

    section("自定义 Display：矩阵对齐");
    let m = Matrix {
        rows: vec![
            vec![1, 234, 5],
            vec![67, 8, 90],
            vec![1, 2, 3456],
        ],
    };
    println!("{m}");

    section("format! 产生 String，不打印");
    let s = format!("{:.2}%", 99.5);
    println!("  format! 产物: {s:?}");

    section("format_args!：零分配（延迟格式化）");
    // format_args! 返回 Arguments，不立即格式化 —— 用于自定义 write。
    let args = format_args!("{} + {} = {}", 2, 3, 5);
    println!("  {args}");
}
