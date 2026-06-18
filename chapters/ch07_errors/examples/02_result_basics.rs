//! 7.2 Result 的基础方法
//!
//! 关键结论：
//! - `Result<T, E>` 表示「成功值 T」或「错误 E」，必须显式处理才能取出值。
//! - 常用方法：`is_ok/is_err`、`ok()/err()`、`unwrap_or(fallback)`、
//!   `unwrap_or_else(closure)`、`unwrap()`（panic）、`expect(msg)`。
//! - `as_ref()/as_mut()`：借用内部值而不消耗 Result。
//! - 未使用的 Result 会触发编译警告 —— 强制你做出决策。
//!
//! 运行：`cargo run -p ch07_errors --example 02_result_basics`

use ch07_errors::section;

/// 模拟「查天气」：随机返回 Ok / Err。
fn get_weather(city: &str) -> Result<&'static str, String> {
    if city.is_empty() {
        Err("城市名不能为空".to_string())
    } else {
        Ok("晴朗 22°C")
    }
}

fn main() {
    section("match：最完整的处理方式");
    match get_weather("北京") {
        Ok(report) => println!("  北京天气: {report}"),
        Err(err) => println!("  查询失败: {err}"),
    }

    section("is_ok / is_err：只想判断成功失败");
    let r = get_weather("上海");
    println!("  Shanghai is_ok = {}", r.is_ok());
    let r = get_weather("");
    println!("  (empty) is_err = {}", r.is_err());

    section("unwrap_or(fallback)：失败时给默认值");
    let fallback = "多云 18°C";
    let report = get_weather("").unwrap_or(fallback);
    println!("  默认天气: {report}");

    section("unwrap_or_else(closure)：失败时才计算默认值");
    // 闭包捕获外部变量，体现「按需计算」的优势（unwrap_or 会先求值默认值）
    let default_factory = String::from("无数据");
    let report = get_weather("")
        .map_err(|e| {
            // 闭包里可以做更复杂的事，比如日志
            println!("  (失败原因: {e})");
            e
        })
        .unwrap_or_else(|_| {
            // 这里「真的需要」才构造字符串
            let mut s = default_factory.clone();
            s.push_str(" (按需生成)");
            s.leak() // 简化演示：把 String 变 'static str
        });
    println!("  闭包默认: {report}");

    section("ok() / err()：转成 Option");
    let ok_opt = get_weather("广州").ok();
    let err_opt = get_weather("").err();
    println!("  ok_opt = {ok_opt:?}");
    println!("  err_opt = {err_opt:?}");

    section("unwrap() / expect(msg)：失败时 panic");
    // 学习代码常用，生产代码慎用！
    let report = get_weather("深圳").unwrap();
    println!("  unwrap: {report}");
    let report = get_weather("杭州").expect("天气查询不应该失败");
    println!("  expect: {report}");

    section("as_ref()：借用而不消耗 Result");
    let r: Result<String, String> = Ok("hello".to_string());
    // 想检查一下结果但不取出 String —— 必须先 as_ref()
    if r.as_ref().is_ok() {
        println!("  仍然能继续使用 r: {:?}", r);
    }

    section("未使用的 Result 会编译警告 —— 强制处理");
    // 这一行如果不写 `let _ =`，clippy/rustc 会警告 unused Result
    // get_weather("重庆");  // ⚠️ warning: unused Result
    let _ = get_weather("重庆");
    println!("  通过 let _ = 显式忽略");
}
