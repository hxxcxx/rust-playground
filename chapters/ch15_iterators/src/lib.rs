//! 第15章 迭代器（Iterators）—— 共享工具与自定义迭代器类型。
//!
//! 本章核心：
//! - `Iterator` trait：只需实现 `next(&mut self) -> Option<Self::Item>`，
//!   就能免费获得几十个「适配器」和「消费者」方法（map/filter/collect/sum...）。
//! - 三类操作：
//!   * 「迭代器」（producer）：产生元素 —— iter() / into_iter() / 0..n。
//!   * 「适配器」（adapter）：变换迭代器，仍是惰性迭代器 —— map/filter/take。
//!   * 「消费者」（consumer）：消耗迭代器，得到结果 —— collect/sum/any/count。
//! - 惰性求值：map/filter 本身不执行任何工作，直到被消费者驱动。
//! - 零开销：迭代器链编译为紧凑循环，与手写循环一样快（甚至更快）。
//! - `IntoIterator`：让类型可以被 for 循环遍历（Vec/&[T]/HashMap/Range 都实现了）。

// =======================================================================
// 工具函数
// =======================================================================

/// 打印带标题的分割线（与前几章风格一致）。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

// =======================================================================
// 自定义迭代器例 1：Range-like 计数器
// =======================================================================

/// 一个「倒数」迭代器：从 n 数到 0。
///
/// 演示「手写 Iterator」最简单的形式 —— 只实现 next()。
pub struct Countdown {
    pub current: i32,
}

impl Countdown {
    pub fn new(start: i32) -> Self {
        Self { current: start }
    }
}

impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= 0 {
            let v = self.current;
            self.current -= 1;
            Some(v)
        } else {
            None // None 表示迭代结束
        }
    }
}

// =======================================================================
// 自定义迭代器例 2：带「过滤」的迭代器
// =======================================================================

/// 一个只产出「偶数」的迭代器（从给定范围内）。
pub struct Evens {
    pub next_val: i32,
    pub end: i32,
}

impl Evens {
    pub fn new(start: i32, end: i32) -> Self {
        // 对齐到第一个 >= start 的偶数。
        let first = if start % 2 == 0 { start } else { start + 1 };
        Self {
            next_val: first,
            end,
        }
    }
}

impl Iterator for Evens {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_val < self.end {
            let v = self.next_val;
            self.next_val += 2; // 每次跳 2，保证都是偶数
            Some(v)
        } else {
            None
        }
    }
}

// =======================================================================
// 自定义迭代器例 3：树形结构的中序遍历（扁平化）
// =======================================================================

/// 简单二叉树节点（用于演示「把树展平成迭代器」）。
#[derive(Debug)]
pub enum Tree {
    Leaf,
    Node {
        value: i32,
        left: Box<Tree>,
        right: Box<Tree>,
    },
}

impl Tree {
    /// 构造一个叶子。
    pub fn leaf() -> Self {
        Tree::Leaf
    }

    /// 构造一个带左右子树的节点。
    pub fn node(value: i32, left: Tree, right: Tree) -> Self {
        Tree::Node {
            value,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// 把树「中序遍历」收集到 Vec（迭代器的另一种实现方式）。
    pub fn in_order(&self) -> Vec<i32> {
        let mut out = Vec::new();
        self.collect_in_order(&mut out);
        out
    }

    fn collect_in_order(&self, out: &mut Vec<i32>) {
        match self {
            Tree::Leaf => {}
            Tree::Node { value, left, right } => {
                left.collect_in_order(out);
                out.push(*value);
                right.collect_in_order(out);
            }
        }
    }
}
