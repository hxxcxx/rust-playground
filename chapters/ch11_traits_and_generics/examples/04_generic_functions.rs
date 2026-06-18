//! 11.4 泛型函数、trait bound、where 子句、单态化
//!
//! 关键结论：
//! - 泛型函数：`fn min<T: Ord>(v1: T, v2: T) -> T` —— T 是「类型参数」。
//! - 单态化（monomorphization）：编译期为每个「具体调用」生成一份专门代码，
//!   例如 `min(1, 2)` 生成 `min_i32`，`min(1.0, 2.0)` 生成 `min_f64` —— 零运行期开销。
//! - trait bound：`<T: Trait>` 限制 T 必须实现哪些 trait，否则无法调用其方法。
//! - 多重 bound：`<T: Read + Write>` 或 `<T: Trait1 + Trait2>`。
//! - `where` 子句：把 bound 挪到签名下方，让函数签名更易读，特别适合复杂约束。
//! - `impl Trait` 写法是 trait bound 的「语法糖」（下一个示例细讲）。
//!
//! 运行：`cargo run -p ch11_traits_and_generics --example 04_generic_functions`

use ch11_traits_and_generics::{Circle, IsShape, Rectangle, section};

// =====================================================================
// 最朴素的泛型函数
// =====================================================================

/// 返回两者中「较小」的一个（需要 T 可比较 → bound: PartialOrd + Copy）。
///
/// 注意：这里用 PartialOrd 而不是 Ord，是为了也能用于 f64。
/// f64 由于存在 NaN（无法全序排序），只实现了 PartialOrd、没有实现 Ord。
fn min<T: PartialOrd + Copy>(v1: T, v2: T) -> T {
    if v1 <= v2 {
        v1
    } else {
        v2
    }
}

/// 返回数组中最大的元素（需要 T 可比较 + 可克隆，避免返回引用）。
fn largest<T: PartialOrd + Copy>(slice: &[T]) -> Option<T> {
    // 空切片返回 None。
    let first = *slice.first()?;
    let mut best = first;
    for &item in slice {
        if item > best {
            best = item;
        }
    }
    Some(best)
}

// =====================================================================
// 用「自定义 trait」作为 bound
// =====================================================================

/// 打印任意 IsShape 的「报告」。
fn report(shape: &impl IsShape) {
    // 这里 shape 必须实现 IsShape —— 否则编译期就报错（提前发现问题）。
    println!("  形状: {} | 面积: {:.2}", shape.name(), shape.area());
}

// =====================================================================
// where 子句：把 bound 挪到后面
// =====================================================================

/// 合并两个「可显示」的值为一个字符串。
///
/// 这里 T: Display + Clone 写在 where 里 —— 适合 bound 较多/较长的场景。
fn merge_display<T, U>(a: T, b: U) -> String
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    format!("{a} + {b}")
}

/// 复杂约束示例：要求 T 同时实现 Ord 和 std::fmt::Debug，
/// 用 where 比 `<T: Ord + std::fmt::Debug>` 在签名里更清爽。
fn top_n<T>(items: &[T], n: usize) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut v = items.to_vec();
    v.sort();
    v.into_iter().rev().take(n).collect()
}

fn main() {
    section("泛型函数 min —— 单态化为不同类型");
    // 同一个 min 函数，对 i32 和 f64 都能调用（编译期各生成一份代码）。
    println!("  min(3, 7)   = {}", min(3, 7));
    println!("  min(3.5, 1.2) = {}", min(3.5, 1.2));
    println!("  min('b','a') = {}", min('b', 'a'));

    section("泛型函数 largest（带 Option）");
    let nums = [3, 9, 2, 7, 5];
    println!("  largest(&[3,9,2,7,5]) = {:?}", largest(&nums));
    let words = ["apple", "fig", "banana"];
    println!("  largest(&words)       = {:?}", largest(&words));
    let empty: [i32; 0] = [];
    println!("  largest(&[])          = {:?}", largest(&empty));

    section("用自定义 trait 作 bound");
    report(&Circle { radius: 2.0 });
    report(&Rectangle { width: 3.0, height: 4.0 });

    section("where 子句让复杂约束更易读");
    println!("  merge_display(42, 'x') = {}", merge_display(42, 'x'));
    println!("  merge_display(\"rust\", 1.5) = {}", merge_display("rust", 1.5));

    section("top_n —— Ord + Clone 的 where 约束");
    let v = vec![5, 1, 8, 3, 9, 2];
    println!("  top_n(&v, 3) = {:?}", top_n(&v, 3));
    let strs = vec!["dog", "cat", "ant", "bee"];
    println!("  top_n(&strs, 2) = {:?}", top_n(&strs, 2));

    section("泛型 vs trait object 的根本区别");
    // 下面这个函数接收「两个 T」，要求它们「类型相同」——
    // 这正是 trait object 不需要、而泛型必须的限制。
    fn same_type_pair<T: IsShape>(a: &T, b: &T) -> f64 {
        a.area() + b.area()
    }
    let c1 = Circle { radius: 1.0 };
    let c2 = Circle { radius: 2.0 };
    println!("  same_type_pair(两个 Circle) = {:.2}", same_type_pair(&c1, &c2));
    // 下面这行会编译失败（类型不同）：
    //   same_type_pair(&Circle{..}, &Rectangle{..});
    println!("  （想混合不同类型 → 用 trait object，见 01_trait_objects）");
}
