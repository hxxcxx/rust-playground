//! 第8章 包和模块 —— 入口。
//!
//! 章节示例：
//! - `01_modules_basic`    —— 内联/嵌套模块、pub 可见性、模块隔离
//! - `02_paths_imports`    —— crate/self/super/:: 路径、use as、pub use
//! - `03_module_files`     —— 调用 src/spores.rs、src/plant_structures/* 中的模块
//! - `04_constants_statics`—— const vs static、可变 static 的限制
//! - `05_attributes`       —— #[cfg]、#[allow]、#![allow]、#[inline]、条件编译
//! - `06_tests`            —— 单元测试 #[test]、#[should_panic]、文档测试

fn main() {
    println!("第8章 包和模块");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch08_crates_modules --example 01_modules_basic");
    println!("  cargo run -p ch08_crates_modules --example 02_paths_imports");
    println!("  cargo run -p ch08_crates_modules --example 03_module_files");
    println!("  cargo run -p ch08_crates_modules --example 04_constants_statics");
    println!("  cargo run -p ch08_crates_modules --example 05_attributes");
    println!("  cargo run -p ch08_crates_modules --example 06_tests");
}
