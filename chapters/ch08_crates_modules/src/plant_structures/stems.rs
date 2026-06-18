//! `plant_structures` 的 `stems` 子模块。
//!
//! 演示「同名文件 + 同名目录」组合：
//! `stems.rs`（本文件）+ `stems/` 目录（含 `xylem.rs` / `phloem.rs`）。

/// 茎：输送水分与养分。
#[derive(Debug, Clone)]
pub struct Stem {
    pub height_cm: f32,
}

/// 木质部：把水从根运到叶。
pub mod xylem {
    /// 简化的运输速率。
    pub fn transport_rate() -> f32 {
        0.5
    }
}

/// 韧皮部：把光合产物从叶运到根。
pub mod phloem {
    /// 简化的运输速率。
    pub fn transport_rate() -> f32 {
        0.3
    }
}
