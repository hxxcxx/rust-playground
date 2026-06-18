//! 12.6 索引：Index / IndexMut
//!
//! 关键结论：
//! - `v[i]` 对应 `Index::index(&v, i)`，返回 `&Output`。
//! - `v[i] = x` 对应 `IndexMut::index_mut(&mut v, i)`，返回 `&mut Output`。
//! - 下标类型「任意」：不限于 usize，可以是元组 `(row, col)`、字符串、自定义类型。
//! - 实现 IndexMut 自动获得「可写索引」能力；只实现 Index 只能读不能写。
//! - `[]` 语法糖等价于 `*Index::index(...)`，越界会 panic（想避免用 get）。
//!
//! 运行：`cargo run -p ch12_operator_overloading --example 06_indexing`

use ch12_operator_overloading::{Grid, section};

fn main() {
    section("Grid：用 (row, col) 元组当下标");
    let mut grid = Grid::new();
    // 这里 [(r, c)] = v 调用的是 IndexMut::index_mut
    grid[(0, 0)] = 1;
    grid[(0, 1)] = 2;
    grid[(1, 0)] = 9;
    grid[(3, 3)] = 99;
    // 这里 [(r, c)] 调用的是 Index::index（只读）
    println!("  grid[(0,0)] = {}", grid[(0, 0)]);
    println!("  grid[(1,0)] = {}", grid[(1, 0)]);
    println!("  grid[(3,3)] = {}", grid[(3, 3)]);

    section("内置类型：Vec / [T; N] / 切片都实现了 Index/IndexMut");
    let mut v = vec![10, 20, 30];
    println!("  v[1] = {}", v[1]);
    v[1] = 99; // IndexMut
    println!("  v[1] = 99 后 = {:?}", v);

    let arr = [100, 200, 300];
    println!("  arr[2] = {}", arr[2]);

    section("下标可以是别的类型：StrKey 用字符串当 key");
    let lookup = NameMap {
        names: vec![("alice", 1), ("bob", 2), ("carol", 3)],
    };
    // lookup["bob"] 直接用字符串索引！
    println!("  lookup[\"bob\"]   = {}", lookup["bob"]);
    println!("  lookup[\"carol\"] = {}", lookup["carol"]);

    section("越界会 panic —— 用 get 安全访问");
    let small = [1, 2, 3];
    // println!("{}", small[10]); // panic: index out of bounds
    match small.get(10) {
        Some(v) => println!("  small.get(10) = {v}"),
        None => println!("  small.get(10) = None（安全）"),
    }

    section("IndexMut 的链式效果：grid[(r,c)] += 1");
    let mut g = Grid::new();
    g[(2, 2)] = 5;
    g[(2, 2)] += 10; // 等价于 *index_mut(&mut g, (2,2)) += 10
    println!("  grid[(2,2)] 从 5 += 10 后 = {}", g[(2, 2)]);
}

/// 演示「字符串下标」：用 &str 作为 Index 的下标类型。
struct NameMap {
    names: Vec<(&'static str, i32)>,
}

impl std::ops::Index<&str> for NameMap {
    type Output = i32;
    fn index(&self, key: &str) -> &i32 {
        // 线性查找（教学用；真实场景用 HashMap）。
        self.names
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v)
            .expect("key not found")
    }
}
