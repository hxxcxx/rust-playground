//! 6.7 类型转换 `as` + 解引用强制转换（deref coercion）
//!
//! 关键结论：
//! - `as` 关键字：数字之间、`bool`/`char`/C 风格枚举 → 整数；不支持反向转换。
//! - 浮点 → 整数：向零取整；超出范围 → 截断到边界（不会 panic）。
//! - 解引用强制转换：`&String` → `&str`、`&Vec<T>` → `&[T]`、`&Box<T>` → `&T`，
//!   `&mut T` → `&T`（这些是自动的，无需写 `as`）。
//! - `as` 用得很少，更多用 `From`/`Into` trait（见第 11 章）。
//!
//! 运行：`cargo run -p ch06_expressions --example 07_casting`

use ch06_expressions::section;

fn main() {
    section("`as` 数字之间转换");
    let x: i32 = 17;
    let idx: usize = x as usize; // i32 → usize
    let small: u8 = x as u8; // 截断：取低 8 位
    println!("  {x} as usize = {idx}");
    println!("  {x} as u8    = {small}");

    section("浮点 → 整数：向零取整 + 超出范围截断");
    let f: f64 = -1.99;
    println!("  {f} as i32 = {}", f as i32); // -1（向零取整）
    let big: f64 = 1e6;
    println!("  {big} as u8 = {}", big as u8); // 255（饱和到边界）

    section("整数 → 浮点");
    let n: i64 = 1_000_000_000_000;
    let f: f64 = n as f64;
    println!("  {n} as f64 = {f}（注意：可能丢精度）");

    section("bool / char / 枚举 → 整数（允许）");
    let b: i32 = true as i32;
    let c: u32 = 'A' as u32;
    println!("  true as i32 = {b}");
    println!("  'A' as u32  = {c}");

    section("整数 → char 不允许：要用 from_u32");
    let n: u32 = 65;
    // let ch = n as char; // ❌ 编译错误：不安全
    let ch = char::from_u32(n); // Option<char>
    println!("  char::from_u32({n}) = {ch:?}");

    section("解引用强制转换（自动）");
    let s: String = String::from("hello");
    let slice: &str = &s; // &String 自动转为 &str
    println!("  &String → &str: {slice}");

    let v: Vec<i32> = vec![1, 2, 3];
    let slice: &[i32] = &v; // &Vec<i32> 自动转为 &[i32]
    println!("  &Vec<i32> → &[i32]: {slice:?}");

    let b: Box<i32> = Box::new(42);
    let r: &i32 = &b; // &Box<i32> 自动转为 &i32
    println!("  &Box<i32> → &i32: {r}");

    section("`&mut T` 自动降级为 `&T`");
    let mut value: i32 = 10;
    let r1: &mut i32 = &mut value;
    let r2: &i32 = r1; // &mut 自动降为 &
    println!("  &mut → &: {r2}");
}
