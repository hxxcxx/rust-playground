//! 5.1 值的引用
//!
//! 关键结论：
//! - 引用是「非拥有所有权的指针」，不影响所指向对象的生命周期 —— 我们称之为「借用」（borrow）。
//! - 共享引用 `&T`：可读不可改，可同时存在任意多个；是 `Copy` 类型。
//! - 可变引用 `&mut T`：可读可改，但同一时刻独占（不能有其它任何引用）；非 `Copy`。
//! - 这相当于编译期强制的「多读者 或 单写者」规则。
//! - 按值传递（move 所有权）vs 按引用传递（仅借用）。
//!
//! 运行：`cargo run -p ch05_references --example 01_value_references`

use ch05_references::{Table, sample_table, section};

// ❌ 反面教材（按值接收 `Table`）：会把整个哈希表移动进函数，
// `for (artist, works) in table` 还会进一步消耗 HashMap 和每个 Vec。
// 函数返回后，调用方的 `table` 就再也不能用了。
// fn show(table: Table) {
//     for (artist, works) in table {
//         ...
//     }
// }

/// ✅ 正确写法：接收共享引用 `&Table`。
/// 遍历 `&HashMap` 会产出对每个条目的「引用」(`&String`, `&Vec<String>`)，
/// 整个过程不发生任何所有权转移，只是传递非拥有的引用。
fn show(table: &Table) {
    // 注意：遍历 &HashMap 时，artist 是 &String，works 是 &Vec<String>
    for (artist, works) in table {
        println!("works by {artist}:");
        for work in works {
            // work 是 &String
            println!("  {work}");
        }
    }
}

/// 需要修改时用「可变引用」`&mut T`：独占访问，可读可写。
/// 这里对每个艺术家的作品做字母排序。
fn sort_works(table: &mut Table) {
    for works in table.values_mut() {
        works.sort(); // Vec::sort 接收 &mut self
    }
}

fn main() {
    section("按值传递会移动所有权（反面教材）");
    println!("如果 show(table: Table) 按值接收，调用后 table 就不可用了；");
    println!("而且 for 循环会消耗整个 HashMap 和其中的 Vec。");

    section("按共享引用传递：借用，不影响所有权");
    let table = sample_table();
    show(&table); // 仅借用，table 仍然有效
    // 调用后仍可继续使用 table：
    assert_eq!(table["Gesualdo"][0], "many madrigals");
    println!("\n(调用 show(&table) 之后，table 仍然可用：)");
    println!("  table[\"Gesualdo\"][0] = {}", table["Gesualdo"][0]);

    section("可变引用：独占访问，可修改");
    let mut table = sample_table();
    sort_works(&mut table); // 独占借用 table 进行修改
    println!("排序后：");
    for (artist, works) in &table {
        println!("  {artist}: {works:?}");
    }

    section("共享引用的「多读者」特性");
    let x = 42_i32;
    let r1 = &x; // 同时存在多个共享引用是允许的
    let r2 = &x;
    let r3 = &x;
    println!("x = {x}, r1 = {r1}, r2 = {r2}, r3 = {r3}");

    section("总结");
    println!("共享引用 &T：可多个、只读、Copy。");
    println!("可变引用 &mut T：独占、可读写、非 Copy。");
    println!("这就是编译期强制的「多读者 或 单写者」规则。");
}
