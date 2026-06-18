//! 10.1 C 风格枚举
//!
//! 关键结论：
//! - `enum Name { V1, V2, ... }` 定义枚举；变体也叫「构造器」。
//! - 内存上：C 风格枚举存为「最小的整数」（默认从 0 开始）。
//! - 可以指定整数值：`Ok = 200, NotFound = 404`。
//! - 枚举 → 整数 用 `as`；整数 → 枚举 **不允许**（避免引入非法值）。
//! - `#[repr(u16)]` 可控制底层整数类型。
//! - 派生 Copy/Clone/Debug/PartialEq/Eq 后可比较 + 打印。
//!
//! 运行：`cargo run -p ch10_enums_patterns --example 01_c_style_enums`

use ch10_enums_patterns::{HttpStatus, TimeUnit, section};

fn main() {
    section("C 风格枚举的基本用法");
    let unit = TimeUnit::Hours;
    println!("  {:?}.plural() = {}", unit, unit.plural());
    println!("  {:?}.singular() = {}", unit, unit.singular());

    section("枚举作为函数返回值（类似 Java 的比较结果）");
    fn compare(n: i32, m: i32) -> TimeUnit {
        // 这里只是借用 TimeUnit 演示，真实场景应返回 std::cmp::Ordering
        let _ = (n, m);
        TimeUnit::Seconds
    }
    let _ = compare(1, 2);
    println!("  (compare 仅演示返回类型)");

    section("指定整数值 + as 转整数");
    let status = HttpStatus::NotFound;
    println!("  {:?} as u16 = {}", status, status as u16);

    section("整数 → 枚举 不允许：必须显式 match 检查");
    let code = 404_u16;
    let parsed = parse_http_status(code);
    println!("  parse_http_status(404) = {parsed:?}");
    let parsed = parse_http_status(500);
    println!("  parse_http_status(500) = {parsed:?} (None —— 非法值)");

    section("枚举也可以有方法（impl 块）");
    let unit = TimeUnit::Days;
    println!("  {} day(s) ago → 用 {}.plural()", 3, unit.singular());

    section("枚举在内存中的大小");
    println!(
        "  size_of::<TimeUnit>() = {} 字节",
        std::mem::size_of::<TimeUnit>()
    );
    println!(
        "  size_of::<HttpStatus>() = {} 字节",
        std::mem::size_of::<HttpStatus>()
    );

    section("派生 PartialEq/Eq 后可以比较");
    let u1 = TimeUnit::Hours;
    let u2 = TimeUnit::Hours;
    println!("  Hours == Hours ? {}", u1 == u2);
}

/// 安全地把 u16 转换为 HttpStatus（手工 match 检查每个合法值）。
fn parse_http_status(n: u16) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}
