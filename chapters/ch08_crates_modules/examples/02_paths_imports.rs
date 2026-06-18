//! 8.2 路径与导入：crate / self / super / :: / use as
//!
//! 关键结论：
//! - 绝对路径：`crate::xxx`（从当前 crate 根开始）、`::xxx`（从外部 crate 根开始）。
//! - 相对路径：`self::xxx`（当前模块）、`super::xxx`（父模块）。
//! - `use`：创建本地别名；可以一次导入多个：`use a::{b, c};`；`use a::*;`。
//! - `use xxx as yyy`：给别名换名（解决冲突或缩短）。
//! - `pub use`：把导入变公开 —— 标准库 prelude 就是一组 pub use。
//! - 模块不会继承父模块的名字：每个模块以「空白」开始（除 prelude 自动导入）。
//!
//! 运行：`cargo run -p ch08_crates_modules --example 02_paths_imports`

use ch08_crates_modules::plant_structures::{
    self, // 导入模块本身
    leaves::Leaf,
    roots::Root,
};
use ch08_crates_modules::section;

// as 重命名：解决冲突
use std::collections::HashMap as StdHashMap;
use std::hash::Hash;

fn main() {
    section("crate:: 绝对路径");
    // 用全路径访问 —— 永远有效，便于代码迁移
    let leaf = crate::plant_structures::leaves::Leaf::new(10.0);
    println!("  leaf area = {} cm²", leaf.area_cm2);

    section("use 后用相对路径");
    let leaf = Leaf::new(20.0);
    let root = Root::new(80.0);
    println!("  leaf area = {} cm²", leaf.area_cm2);
    println!("  root depth = {} cm", root.depth_cm);

    section("use module 重导入子模块");
    // 因为前面有 `use plant_structures::{self, ...}`
    let leaf = plant_structures::leaves::Leaf::new(5.0);
    println!("  通过 plant_structures::leaves: {}", leaf.area_cm2);

    section("`use as` 解决命名冲突");
    let mut map: StdHashMap<String, i32> = StdHashMap::new();
    map.insert("a".into(), 1);
    println!("  StdHashMap: {map:?}");

    section("一次导入多个项（花括号）");
    use std::collections::{HashMap, HashSet};
    let _h1: HashMap<i32, i32> = HashMap::new();
    let _h2: HashSet<i32> = HashSet::new();
    println!("  HashMap + HashSet 同时导入成功");

    section("`use Trait` 后才能调用其方法");
    // Hash trait 的方法（间接通过 HashMap）—— 这里仅演示导入
    fn check_hash<T: Hash>(_t: &T) -> bool {
        true
    }
    println!("  check_hash(\"abc\") = {}", check_hash(&"abc"));

    section("标准前置模块 prelude 自动导入");
    // Vec、String、Option、Result 等不需要 use —— 它们在 std::prelude::v1 里
    let v: Vec<i32> = vec![1, 2, 3];
    let s: String = String::from("hello");
    let o: Option<i32> = Some(42);
    let r: Result<i32, &str> = Ok(0);
    println!("  prelude 类型: {v:?} / {s} / {o:?} / {r:?}");
}
