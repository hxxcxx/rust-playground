//! 第8章 包和模块 —— 根模块（lib.rs）。
//!
//! 本章核心：
//! - Crate（包）：编译单元，分二进制 crate 和库 crate。
//! - Module（模块）：crate 内的命名空间，用于组织代码。
//! - `pub`：可见性控制 —— `pub`/`pub(crate)`/`pub(super)`/`pub(in path)`。
//! - 路径：`crate::`（绝对）、`self::`（当前）、`super::`（父）、`::`（外部 crate 根）。
//! - `use`：导入别名；`use ... as ...`；`pub use`（重新导出）。
//! - 模块组织方式：内联 `mod {}` / `mod x;` + `x.rs` / `mod x;` + `x/mod.rs`。
//! - 项声明：`fn`/`struct`/`enum`/`const`/`static`/`trait`/`impl` 都能放进模块。
//! - 标准前置模块（prelude）：`Vec`/`Result` 等自动导入。

/// 共享工具：打印带标题的分割线。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

// === 三种模块组织形式 ===

/// 内联模块：模块体直接写在 `mod x { ... }` 里。
pub mod inline_plant {
    /// 模块内的公共函数。
    pub fn describe() -> &'static str {
        "内联模块：模块体直接写在花括号里"
    }

    /// 模块内的私有函数：只有本模块及子模块能访问。
    fn private_helper() -> i32 {
        42
    }

    /// 子模块可以访问父模块的私有项（用 `super::`）。
    pub mod child {
        /// 调用父模块的私有函数 —— 演示「模块隔离」的可见性边界。
        pub fn use_parent_private() -> i32 {
            super::private_helper()
        }
    }
}

/// 文件分离模块：`mod spores;` → Rust 加载 `src/spores.rs`。
pub mod spores;

/// 目录形式模块：`mod plant_structures;` → Rust 加载 `src/plant_structures/mod.rs`。
pub mod plant_structures;

// === 常量与静态变量 ===

/// 公共常量：编译期常量，每次使用都被内联进代码（类似 C 的 #define）。
pub const ROOM_TEMPERATURE_C: f64 = 20.0;

/// 公共静态变量：进程级单例，所有线程共享同一份。
/// 不可变静态变量是线程安全的。
pub static GRAVITY_ACCEL: f64 = 9.81;

// === 类型关联常量演示（章节里用） ===

/// 重新导出：把子模块的符号「提升」到 crate 根，方便外部使用。
pub use plant_structures::{Leaf, Root};

/// 公共转发函数：演示「pub(crate) 函数只能在 crate 内部访问」。
/// 外部 example 通过这个 pub 包装器间接调用 pub(crate) 的 spores::genes。
pub fn spore_genes(spore: &spores::Spore) -> Vec<u32> {
    spores::genes(spore)
}

/// 模块的 `prelude`：约定俗成的「常用导入集合」，外部可用 `use ch08::*;`。
pub mod prelude {
    pub use crate::plant_structures::stems::Stem;
    pub use crate::plant_structures::{Leaf, Root};
    pub use crate::spores::Spore;
}
