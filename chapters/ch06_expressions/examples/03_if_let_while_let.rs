//! 6.3 `if let` 与 `while let`
//!
//! 关键结论：
//! - `if let PAT = EXPR { ... } else { ... }`：单分支模式匹配的简写。
//!   等价于 `match EXPR { PAT => {...}, _ => {...} }`。
//! - `while let PAT = EXPR { ... }`：每次循环开始匹配；不匹配则退出循环。
//! - 它们属于「可反驳模式」（refutable）—— 可能匹配失败。
//! - 而 `let PAT = ...`/函数参数/`for` 必须用「不可反驳模式」（irrefutable）。
//!
//! 运行：`cargo run -p ch06_expressions --example 03_if_let_while_let`

use ch06_expressions::section;

fn main() {
    section("if let：只关心一种情况时的简写");
    let session_cookie: Option<&str> = Some("session-abc");
    if let Some(cookie) = session_cookie {
        println!("  restoring session for cookie = {cookie}");
    } else {
        println!("  no session cookie, fresh login");
    }

    // 等价的 match 写法（明显更冗长）：
    match session_cookie {
        Some(cookie) => println!("  (match 版) restore {cookie}"),
        None => println!("  (match 版) no cookie"),
    }

    section("if let 配合 Result：拿到值或提前返回");
    let parsed: Result<i32, &str> = Ok(42);
    if let Ok(n) = parsed {
        println!("  got number: {n}");
    }

    section("while let：处理「流」式数据，直到拿到 None/不匹配");
    // 模拟一个简易迭代器：每调用一次 pop 一个值
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("  popped {top}");
    }
    println!("  stack empty: {:?}", stack);

    section("while let Err：重试直到成功");
    let mut attempts = 0u32;
    let mut fake_op = || {
        attempts += 1;
        if attempts < 3 {
            Err(format!("attempt {attempts} failed"))
        } else {
            Ok(format!("attempt {attempts} ok"))
        }
    };
    while let Err(reason) = fake_op() {
        println!("  retrying because: {reason}");
    }
    println!("  finally succeeded after {attempts} attempts");
}
