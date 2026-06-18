//! 8.5 属性：#[cfg]、#[allow]、#![allow]、#[inline]、条件编译
//!
//! 关键结论：
//! - 属性以 `#[...]` 开头，附加到「下一项」；`#![...]` 附加到「包含它的项」。
//! - `#[cfg(...)]`：条件编译 —— `target_os`、`feature`、`debug_assertions` 等。
//! - `#[allow(lint)]`：抑制某个 lint 警告；`#![allow]` 整个 crate 抑制。
//! - `#[inline]`：建议编译器内联；`#[inline(always)]` 强制；`#[inline(never)]` 禁止。
//! - `#[test]`：标记测试函数（见下一个 example）。
//! - `#![feature(...)]`：启用 nightly 不稳定特性（仅 nightly 工具链）。
//!
//! 运行：`cargo run -p ch08_crates_modules --example 05_attributes`

use ch08_crates_modules::section;

fn main() {
    section("条件编译 #[cfg]");
    print_platform();

    section("debug_assertions：仅 debug 构建启用");
    debug_only_check();

    section("feature flag：通过 cargo build --feature xxx 启用");
    feature_check();

    section("#[allow] 抑制 lint");
    // 例如 `#[allow(non_camel_case_types)]` 让某个类型用 snake_case 不报警
    #[allow(non_camel_case_types)]
    struct git_revspec {
        _opaque: (),
    }
    let _ = git_revspec { _opaque: () };
    println!("  non_camel_case 类型被允许");

    section("#[inline]：内联提示");
    let r = add_inline(3, 4);
    println!("  add_inline(3, 4) = {r}");

    section("#! 形式：附加到 crate 而非下一项");
    println!("  // lib.rs 顶部：#![allow(unused)] → 整个 crate 允许未使用警告");
    println!("  // 单项：#[allow(unused)] → 仅下一项允许");

    section("常用 #[cfg] 选项表");
    println!("  target_os = \"windows\" / \"linux\" / \"macos\" / \"android\"");
    println!("  target_arch = \"x86_64\" / \"aarch64\" / \"wasm32\"");
    println!("  unix / windows");
    println!("  debug_assertions  (非 release 构建)");
    println!("  feature = \"xxx\"");
    println!("  all(A, B) / any(A, B) / not(A)");
}

/// 条件编译：只在特定平台编译
#[cfg(target_os = "windows")]
fn print_platform() {
    println!("  当前平台: Windows");
}

#[cfg(target_os = "linux")]
fn print_platform() {
    println!("  当前平台: Linux");
}

#[cfg(target_os = "macos")]
fn print_platform() {
    println!("  当前平台: macOS");
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn print_platform() {
    println!("  当前平台: 其他（非 Windows/Linux/macOS）");
}

/// `debug_assertions`：debug 构建启用，release 关闭
#[cfg(debug_assertions)]
fn debug_only_check() {
    println!("  debug 模式：执行额外检查");
}

#[cfg(not(debug_assertions))]
fn debug_only_check() {
    println!("  release 模式：跳过 debug 检查");
}

/// 用 feature flag 控制：cargo build --feature extra
#[cfg(feature = "extra")]
fn feature_check() {
    println!("  feature \"extra\" 已启用");
}

#[cfg(not(feature = "extra"))]
fn feature_check() {
    println!("  feature \"extra\" 未启用（默认）");
}

/// `#[inline]` 给编译器的内联建议
#[inline]
fn add_inline(a: i32, b: i32) -> i32 {
    a + b
}
