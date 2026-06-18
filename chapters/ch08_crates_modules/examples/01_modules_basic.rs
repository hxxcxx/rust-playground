//! 8.1 模块基础：嵌套模块 + pub 可见性 + 模块隔离
//!
//! 关键结论：
//! - `mod name { ... }` 定义内联模块；`pub mod` 让它对外可见。
//! - 模块内项默认私有，`pub` 才能跨模块访问；私有项在「本模块及其后代模块」中可用。
//! - `pub(crate)`：仅本 crate 内可用（不暴露给外部 crate）。
//! - `pub(super)`：仅父模块可用。
//! - Rust 按「模块」而非「类」实施访问控制 —— 比 Java/C++ 的 friend 机制更灵活。
//!
//! 运行：`cargo run -p ch08_crates_modules --example 01_modules_basic`

use ch08_crates_modules::{inline_plant, section};

fn main() {
    section("内联模块 inline_plant");
    println!("  describe: {}", inline_plant::describe());

    section("子模块访问父模块的私有函数");
    // child 是 inline_plant 的子模块，可以访问 inline_plant::private_helper
    let n = inline_plant::child::use_parent_private();
    println!("  child → 父私有函数: {n}");

    section("Rust 按「模块」而非「类」做访问控制");
    println!("  → 同模块内的多个类型可以互相访问私有字段");
    println!("  → 比 C++ 的 friend 声明更简单");

    section("pub(crate)：仅 crate 内可见");
    let spore = ch08_crates_modules::spores::produce_spore(1);
    // pub(crate) 函数 spores::genes 外部 crate 不能直接调用，
    // lib.rs 提供了一个 pub 包装器 spore_genes 来演示。
    let genes = ch08_crates_modules::spore_genes(&spore);
    println!("  crate 内调用 pub(crate) genes (经包装): {genes:?}");
}

// 演示「同 crate 内调用 pub(crate) 函数」—— 这个模块在 example 二进制里，
// 属于「另一个 crate」，所以不能直接调用 ch08_crates_modules::spores::genes。
// 真正的同 crate 访问发生在 lib.rs 内部，例如 plant_structures/mod.rs 中。
mod ch08_crates_module_inner {
    #[allow(dead_code)]
    pub fn note() -> &'static str {
        "本模块属于 example 这个独立 crate，无法访问 lib crate 的 pub(crate) 项"
    }
}
