//! 第16章 集合（Collections）—— 共享类型与示例。
//!
//! 本章核心：std::collections 里的常用容器，以及它们各自的适用场景。
//!
//! - `Vec<T>`           —— 动态数组（栈式追加最快，随机访问 O(1)）。默认选择。
//! - `VecDeque<T>`      —— 双端队列（头尾都 O(1) push/pop）。滑动窗口/队列。
//! - `LinkedList<T>`    —— 双向链表（极少用；Rust 里几乎总被 Vec 替代）。
//! - `HashMap<K, V>`    —— 哈希表（O(1) 平均，无序）。键需 Hash + Eq。
//! - `BTreeMap<K, V>`   —— 有序映射（O(log n)，按 key 排序）。键需 Ord。
//! - `HashSet<T>`/`BTreeSet<T>` —— 对应的「集合」（只存 key，无 value）。
//! - `BinaryHeap<T>`    —— 二叉堆（最大堆，O(log n) push/pop，O(1) peek）。
//!
//! 选择口诀：
//! - 顺序、随机访问、栈式追加 → Vec
//! - 两端都要 push/pop → VecDeque
//! - 查表（key→value）→ HashMap（无序）/ BTreeMap（要排序）
//! - 去重 / 集合运算 → HashSet / BTreeSet
//! - 取最大值 → BinaryHeap

use std::collections::HashMap;

// =======================================================================
// 工具函数
// =======================================================================

/// 打印带标题的分割线（与前几章风格一致）。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

// =======================================================================
// 示例类型：用于演示「自定义类型作 HashMap key」
// =======================================================================

/// 一个坐标点：演示「实现 Hash + Eq 后作 HashMap/HashSet 的 key」。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// =======================================================================
// 示例：BTreeMap 的有序性用于「事件时间线」
// =======================================================================

/// 一个简单的事件：演示存进 BTreeMap 后按时间排序。
#[derive(Debug, Clone)]
pub struct Event {
    pub name: &'static str,
    pub detail: String,
}

/// 构造一个「时间 → 事件」的示例 BTreeMap（顺序由调用方维护）。
pub fn sample_timeline() -> std::collections::BTreeMap<u64, Event> {
    let mut map = std::collections::BTreeMap::new();
    map.insert(
        30,
        Event { name: "起床", detail: "闹钟响了".into() },
    );
    map.insert(
        10,
        Event { name: "入睡", detail: "结束一天".into() },
    );
    map.insert(
        20,
        Event { name: "做梦", detail: "进入 REM".into() },
    );
    map
}

// =======================================================================
// 示例：用 HashMap 做「字符频率统计」的工具
// =======================================================================

/// 统计字符串中每个字符的出现次数（演示 HashMap 经典用法）。
pub fn char_frequency(text: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for c in text.chars() {
        // entry().or_insert(0) —— 不存在则插入 0，返回 &mut。
        *freq.entry(c).or_insert(0) += 1;
    }
    freq
}
