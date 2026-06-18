//! 13.5 Default —— 提供默认值
//!
//! 关键结论：
//! - `Default::default() -> Self`：返回类型的「零值/默认值」。
//! - 派生 `#[derive(Default)]` 后，每个字段用各自的 default。
//! - 经典用途：
//!   * 配置/选项结构体：只覆盖关心的字段，其余用默认。
//!   * 泛型工厂：`fn new() -> T: Default { T::default() }`。
//!   * `unwrap_or_default()`：Option/Result 为空时返回默认。
//! - 结构体更新语法 `..Default::default()`：只填几个字段，其余继承默认。
//!
//! 运行：`cargo run -p ch13_utility_traits --example 05_default`

use ch13_utility_traits::{ServerConfig, section};

fn main() {
    section("基本类型都有 Default");
    println!("  i32::default()    = {}", i32::default());   // 0
    println!("  f64::default()    = {}", f64::default());   // 0.0
    println!("  String::default() = {:?}", String::default()); // ""
    println!("  Vec::<i32>::default() = {:?}", Vec::<i32>::default()); // []
    println!("  bool::default()   = {}", bool::default());  // false
    println!("  Option::<i32>::default() = {:?}", Option::<i32>::default()); // None

    section("派生 Default：每个字段用各自的默认");
    #[derive(Debug, Default)]
    struct Settings {
        debug: bool,           // 默认 false
        retries: u32,          // 默认 0
        label: String,         // 默认 ""
    }
    let s = Settings::default();
    println!("  {s:?}");

    section("结构体更新语法：..Default::default()");
    // 只覆盖 host 和 port，其余字段用默认。
    let cfg = ServerConfig {
        host: "0.0.0.0".to_string(),
        port: 3000,
        ..Default::default()
    };
    println!("  自定义 host/port: {cfg:?}");

    section("Default 用于「可配置 + 提供默认」的 API");
    let default_cfg = ServerConfig::default();
    let custom_cfg = ServerConfig {
        max_connections: 1000,
        ..ServerConfig::default()
    };
    println!("  默认配置:     {default_cfg:?}");
    println!("  自定义连接数: {custom_cfg:?}");

    section("unwrap_or_default：空值时返回默认");
    let nums: Vec<i32> = vec![];
    let sum: i32 = nums.first().copied().unwrap_or_default();
    println!("  空切片 first().unwrap_or_default() = {sum}");

    let maybe_name: Option<String> = None;
    // 演示用：教学 None 的 unwrap_or_default 行为，clippy 会提醒字面量 None。
    #[allow(clippy::unnecessary_literal_unwrap)]
    let name = maybe_name.unwrap_or_default();
    println!("  None.unwrap_or_default() = {name:?}（空 String）");

    section("泛型工厂：要求 T: Default");
    fn make_default<T: Default>() -> T {
        T::default()
    }
    let n: i32 = make_default();
    let s: String = make_default();
    println!("  make_default::<i32>()    = {n}");
    println!("  make_default::<String>() = {s:?}");

    section("手写 Default：定制「业务默认」");
    // 派生的 Default 让 retries=0，但业务上希望默认 3 次重试。
    let app = AppConfig::default();
    println!("  AppConfig::default() = {app:?}");
}

/// 演示「手写 Default」：覆盖派生行为。
#[derive(Debug, Clone)]
struct AppConfig {
    retries: u32,
    timeout_secs: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        // 业务上希望「默认重试 3 次、超时 60 秒」
        Self {
            retries: 3,
            timeout_secs: 60,
        }
    }
}
