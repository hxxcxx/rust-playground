//! 8.3 文件分离的模块：调用 src/spores.rs 与 src/plant_structures/* 中的模块
//!
//! 关键结论：
//! - `mod xxx;`（不带花括号）→ Rust 自动加载：
//!   - `src/xxx.rs`，或
//!   - `src/xxx/mod.rs`
//! - 二者只能存在一个，否则报错。
//! - 「同名文件 + 同名目录」组合：`stems.rs` + `stems/` 目录（含子模块）。
//! - 模块在文件里不需要任何样板，直接写项即可。
//! - 「库 + 可执行」共存：lib.rs 是库 crate 根，main.rs 是二进制 crate 根。
//!
//! 运行：`cargo run -p ch08_crates_modules --example 03_module_files`

use ch08_crates_modules::plant_structures::{leaves::Leaf, roots::Root, stems::Stem};
use ch08_crates_modules::prelude::*; // 一次性导入「常用集合」
use ch08_crates_modules::section;
use ch08_crates_modules::spores;
use ch08_crates_modules::{Leaf as TopLeaf, Root as TopRoot}; // pub use 重导出

fn main() {
    section("调用 src/spores.rs 中的单文件模块");
    let spore = spores::produce_spore(42);
    println!("  spore = {spore:?}");

    section("调用 src/plant_structures/ 目录形式模块");
    let leaf = Leaf::new(15.5);
    let root = Root::new(60.0);
    let stem = Stem { height_cm: 120.0 };
    println!("  leaf = {leaf:?}");
    println!("  root = {root:?} (deep? {})", root.is_deep_enough());
    println!("  stem = {stem:?}");

    section("调用 stems.rs + stems/ 目录的组合");
    // stems.rs 定义了 Stem，同时声明了子模块 xylem 和 phloem
    println!(
        "  xylem transport = {}",
        ch08_crates_modules::plant_structures::stems::xylem::transport_rate()
    );
    println!(
        "  phloem transport = {}",
        ch08_crates_modules::plant_structures::stems::phloem::transport_rate()
    );

    section("pub use 重新导出：用 crate 顶层路径访问");
    let _leaf = TopLeaf::new(7.0);
    let _root = TopRoot::new(30.0);
    println!("  通过 pub use 提升到 crate 根的别名可用");

    section("use prelude::*：导入约定俗成的「常用集合」");
    let _leaf: Leaf = Leaf::new(1.0);
    let _root: Root = Root::new(2.0);
    let _stem: Stem = Stem { height_cm: 3.0 };
    let _spore: Spore = spores::produce_spore(0);
    println!("  prelude 中的类型一次性可用");

    section("库与可执行并存：lib.rs 是库，main.rs/examples 是可执行");
    println!("  → src/lib.rs 编译为库 crate ch08_crates_modules");
    println!("  → src/main.rs 与 examples/*.rs 编译为独立的二进制 crate");
    println!("  → 二进制 crate 通过 use ch08_crates_modules::xxx 来用库");
}
