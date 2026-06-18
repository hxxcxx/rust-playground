//! 模块组织示例：`mod spores;` 引用本文件 `spores.rs`。
//!
//! 关键点：
//! - 单文件模块：`mod spores;` 让 Rust 加载 `spores.rs`。
//! - 文件内不需要任何「我是模块」的样板代码，直接写项即可。
//! - `pub`/私有的规则与内联模块完全一致。

/// 孢子结构体（公共）。
#[derive(Debug, Clone)]
pub struct Spore {
    pub id: u32,
    pub generation: u32,
}

/// 模拟减数分裂产生孢子。
pub fn produce_spore(id: u32) -> Spore {
    Spore { id, generation: 0 }
}

/// 仅在 crate 内可见（外部包无法访问）。
pub(crate) fn genes(spore: &Spore) -> Vec<u32> {
    vec![spore.id, spore.generation]
}

/// 完全私有，只有本模块及其子模块可用。
fn recombine() {
    // 私有函数：模拟基因重组
}
