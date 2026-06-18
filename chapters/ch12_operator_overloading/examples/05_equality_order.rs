//! 12.5 相等与排序：PartialEq / Eq / PartialOrd / Ord / Hash
//!
//! 关键结论：
//! - `==` `!=` 对应 `PartialEq`；`<` `>` `<=` `>=` 对应 `PartialOrd`。
//! - `PartialEq::eq(&self, &other) -> bool` 是核心，`ne` 有默认实现（取反 eq）。
//! - `Eq` 是 `PartialEq` 的「标记 trait」：表示「相等关系是自反的」
//!   （a == a 永远成立）。浮点数因为有 NaN，不实现 Eq，只实现 PartialEq。
//! - `Ord`（全序）要求实现 `cmp`；`PartialOrd` 只要求 `partial_cmp`（可能返回 None）。
//! - 大多数情况：直接 `#[derive(PartialEq, Eq, PartialOrd, Ord)]` 最方便。
//! - 想进 HashMap/HashSet 当 key？需要 `Hash`（通常也 derive）。
//! - 自定义比较：手写 impl，常见于「按某字段排序」「忽略大小写比较」。
//!
//! 运行：`cargo run -p ch12_operator_overloading --example 05_equality_order`

use ch12_operator_overloading::section;
use std::cmp::Ordering;

fn main() {
    section("派生 PartialEq：字段全相等才相等");
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 1, y: 3 };
    println!("  p1 == p2 ? {}", p1 == p2); // true
    println!("  p1 == p3 ? {}", p1 == p3); // false

    section("浮点数：PartialEq 有，Eq 没有（NaN != NaN）");
    let nan = f64::NAN;
    // clippy 会抱怨两边表达式相同 —— 这里正是要演示 NaN != NaN，保留它。
    #[allow(clippy::eq_op)]
    let is_equal = nan == nan;
    println!("  NaN == NaN ? {is_equal}"); // false！
    // 下面这行无法编译 —— f64 没实现 Eq：
    //   fn needs_eq<T: Eq>() {}
    //   needs_eq::<f64>();
    println!("  （所以 f64 不能放进需要 Eq 的容器，如 BTreeMap 的 key）");

    section("自定义 PartialEq：忽略大小写比较字符串");
    let s1 = CaseInsensitiveStr("Hello");
    let s2 = CaseInsensitiveStr("HELLO");
    println!("  Hello == HELLO (忽略大小写) ? {}", s1 == s2);

    section("自定义 Ord：按 y 优先排序");
    let mut pts = vec![
        Point { x: 5, y: 1 },
        Point { x: 1, y: 5 },
        Point { x: 3, y: 3 },
    ];
    pts.sort(); // 用 Point 自己的 Ord（按 y）
    println!("  按 y 排序后: {:?}", pts);

    section("用 cmp + Ordering 做三分比较");
    let a = 5;
    let b = 10;
    match a.cmp(&b) {
        Ordering::Less => println!("  {a} < {b}"),
        Ordering::Equal => println!("  {a} == {b}"),
        Ordering::Greater => println!("  {a} > {b}"),
    }

    section("Hash：让自定义类型能进 HashMap");
    use std::collections::HashMap;
    let mut map: HashMap<Point, &str> = HashMap::new();
    map.insert(Point { x: 0, y: 0 }, "origin");
    map.insert(Point { x: 1, y: 1 }, "diag");
    println!("  map[Point(0,0)] = {:?}", map.get(&Point { x: 0, y: 0 }));

    section("sort_by / sort_by_key：不实现 Ord 也能临时排序");
    let mut nums = vec![5, 2, 8, 1, 9];
    nums.sort_by(|a, b| b.cmp(a)); // 降序
    println!("  降序: {:?}", nums);
    let mut words = vec!["banana", "apple", "cherry"];
    words.sort_by_key(|w| w.len()); // 按长度
    println!("  按长度: {:?}", words);
}

/// 一个二维点：派生全套比较 trait（最常见做法）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

// 手写 Ord：按 y 优先排序（覆盖 derive 的字段顺序）。
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        // 先比 y，y 相同再比 x。
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

/// 忽略大小写的字符串包装类型：手写 PartialEq。
#[derive(Debug)]
struct CaseInsensitiveStr<'a>(&'a str);

impl PartialEq for CaseInsensitiveStr<'_> {
    fn eq(&self, other: &Self) -> bool {
        // 把两边都转小写后比较 —— 自定义相等语义。
        self.0.eq_ignore_ascii_case(other.0)
    }
}
