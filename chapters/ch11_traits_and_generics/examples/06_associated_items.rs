//! 11.6 关联项 —— 关联类型 / 关联常量 / 关联函数
//!
//! 关键结论：
//! - 「关联类型」（associated type）：trait 上的「占位类型」，由实现者指定。
//!   好处：调用方不必每次写 `Iter<T>`，类型更简洁；一个类型对每个 trait 只能有一份实现。
//!   经典案例：标准库 `Iterator` 的 `type Item`。
//! - 「关联常量」（associated constant）：trait 上由实现者提供的「编译期常量」。
//! - 「关联函数」（associated function）：trait 里不带 self 的函数（类似静态方法）。
//! - 关联类型 vs 泛型参数：
//!   * 关联类型：实现者「唯一决定」，每个 (Type, Trait) 只有一份 impl。
//!   * 泛型 trait：调用方可以「为同一类型实现多次」（如 From<T> 可实现多种 T）。
//!
//! 运行：`cargo run -p ch11_traits_and_generics --example 06_associated_items`

use ch11_traits_and_generics::{
    Circle, Counter, HasZeroDimension, MyIterator, NewsArticle, Summary, section,
};

// =====================================================================
// 关联类型 + 关联函数 + 关联常量：一个「容器」trait
// =====================================================================

/// 一个容器：内部装「某种」元素，元素类型由关联类型 Item 决定。
pub trait Container {
    /// 容器里元素的类型（实现者决定）。
    type Item;

    /// 关联常量：容器的「最大容量」上限。
    const MAX_CAPACITY: usize;

    /// 关联函数（无 self）：创建一个空的容器（类似静态工厂方法）。
    fn empty() -> Self;

    /// 取出第 index 个元素的引用。
    fn at(&self, index: usize) -> Option<&Self::Item>;

    /// 长度。
    fn len(&self) -> usize;

    /// 是否为空（有 len 就应该有 is_empty，clippy 也会提醒）。
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 用关联常量判断是否「满了」。
    fn is_full(&self) -> bool {
        self.len() >= Self::MAX_CAPACITY
    }
}

/// 一个固定容量的环形缓冲（简化版），演示 Container 实现。
pub struct FixedBuf<T, const N: usize> {
    data: [Option<T>; N],
    len: usize,
}

// 注意：impl 块里「同时」使用了关联类型 Item、关联常量 MAX_CAPACITY、关联函数 empty。
impl<T: Clone, const N: usize> Container for FixedBuf<T, N> {
    type Item = T;
    const MAX_CAPACITY: usize = N;

    fn empty() -> Self {
        // const N 已知，可以用 [const; N] 初始化数组（每个槽位 None）。
        Self {
            data: std::array::from_fn(|_| None),
            len: 0,
        }
    }

    fn at(&self, index: usize) -> Option<&Self::Item> {
        self.data.get(index).and_then(|slot| slot.as_ref())
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl<T: Clone, const N: usize> FixedBuf<T, N> {
    fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N {
            return Err(value);
        }
        self.data[self.len] = Some(value);
        self.len += 1;
        Ok(())
    }
}

// =====================================================================
// 关联类型 vs 泛型 trait：什么时候用哪个？
// =====================================================================

/// 泛型 trait：可以「为同一类型实现多次」（参数不同）。
/// 标准库的 From<T>、Add<Rhs> 都是这种 —— 因为一个类型可以从多种来源转换。
pub trait FromExample<T> {
    fn from_example(value: T) -> Self;
}

/// 一个温度值。
#[derive(Debug, Clone)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone)]
pub struct Fahrenheit(pub f64);

// Celsius 可以「从 f64」转换
impl FromExample<f64> for Celsius {
    fn from_example(value: f64) -> Self {
        Celsius(value)
    }
}
// 也可以「从 Fahrenheit」转换 —— 同一类型两次 impl，参数不同（泛型 trait 允许）。
impl FromExample<Fahrenheit> for Celsius {
    fn from_example(Fahrenheit(f): Fahrenheit) -> Self {
        Celsius((f - 32.0) * 5.0 / 9.0)
    }
}

fn main() {
    section("关联类型：MyIterator 的 Counter");
    let mut counter = Counter::new(1, 5);
    println!("  Counter::Item 是 i32（由实现者决定）");
    while let Some(n) = counter.next() {
        println!("    next() = {n}");
    }

    section("关联类型 + 关联常量 + 关联函数：Container");
    let mut buf: FixedBuf<&str, 3> = FixedBuf::empty(); // 关联函数 empty()
    println!("  MAX_CAPACITY = {}（关联常量）", FixedBuf::<&str, 3>::MAX_CAPACITY);
    println!("  一开始 is_full = {}", buf.is_full());
    buf.push("a").unwrap();
    buf.push("b").unwrap();
    buf.push("c").unwrap();
    println!("  塞满 3 个后 is_full = {}", buf.is_full());
    println!("  buf.at(1) = {:?}", buf.at(1));
    println!("  buf.at(99) = {:?}", buf.at(99));

    section("默认方法使用关联常量（is_full 用了 MAX_CAPACITY）");
    let full = buf.is_full();
    println!("  容量 = {}, 长度 = {}, is_full = {}", 3, buf.len(), full);

    section("关联类型 vs 泛型 trait");
    let c1 = Celsius::from_example(36.5_f64);
    let c2 = Celsius::from_example(Fahrenheit(98.6));
    println!("  from f64        = {:?}", c1);
    println!("  from Fahrenheit = {:?}", c2);
    println!("  （Celsius 对 FromExample 实现了两次 —— 泛型 trait 允许）");

    section("关联类型的实际用途：标准库 Iterator / Deref / Add 等");
    // Iterator::Item、Deref::Target、Add::Output 都是关联类型。
    let v = [1, 2, 3];
    // Vec<u8> 的 Deref::Target = [u8]，可以隐式解引用为切片。
    let _sum: i32 = v.iter().sum(); // iter() 用了 Iterator::Item = &i32
    println!("  sum([1,2,3]) = {_sum}");

    section("Summary（前一章定义的 trait）关联到具体类型");
    let news = NewsArticle {
        headline: "Rust 2024 edition GA".into(),
        location: "上海".into(),
    };
    println!("  news.summarize() = {}", news.summarize());

    section("HasZeroDimension：关联常量 ZERO / UNIT");
    // 关联常量可以直接用 Type::CONST 访问（不需要实例）。
    println!("  Circle::ZERO = {}, Circle::UNIT = {}", Circle::ZERO, Circle::UNIT);
}
