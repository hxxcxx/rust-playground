//! 目录形式的模块：`plant_structures/` 目录 + `mod.rs`。
//!
//! Rust 看到 `mod plant_structures;` 时，会查找：
//!   - `plant_structures.rs`，或
//!   - `plant_structures/mod.rs`（本文件）
//! 二者只能存在一个。
//!
//! 在 `mod.rs` 中再用 `pub mod roots;` 声明子模块。

/// 公开三个子模块 —— 各自放在 `roots.rs`/`stems.rs`/`leaves.rs`。
pub mod leaves;
pub mod roots;
pub mod stems;

/// 重新导出（pub use）：把子模块中的类型「提升」到本模块路径。
/// 这样外部可以用 `plant_structures::Leaf` 而不是 `plant_structures::leaves::Leaf`。
pub use leaves::Leaf;
pub use roots::Root;
