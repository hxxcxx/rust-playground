//! 第10章 枚举和模式 —— 共享工具与示例类型。
//!
//! 本章核心：
//! - 枚举（enum）：C 风格 + 含数据变体 + 泛型枚举（Option/Result 就是）。
//! - 三种变体形式：无数据（类单元）/ 元组变体 / 结构体变体。
//! - 模式匹配：字面量 / 范围 / 变量 / 通配符 / 元组 / 结构体 / 数组 / 切片 / 引用。
//! - 高级特性：匹配守卫 `if`、多重模式 `|`、范围 `..=`、绑定 `@`、`ref`/`ref mut`。
//! - 不可反驳模式 vs 可反驳模式（仅后者可用于 match 分支）。
//! - 实战：用 enum 表示 JSON / 二叉树，配合 match 实现复杂逻辑。

use std::collections::HashMap;

/// 打印带标题的分割线。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

// =======================================================================
// C 风格枚举
// =======================================================================

/// 时间单位：演示 C 风格枚举 + 派生 trait + impl 方法。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    /// 返回复数名词。
    pub fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    /// 返回单数名词（去掉末尾的 s）。
    pub fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

/// HTTP 状态码：演示带「整数值」的 C 风格枚举。
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

// =======================================================================
// 含数据的枚举
// =======================================================================

/// 故意取整的时间戳：演示含数据的枚举变体。
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

// =======================================================================
// 用枚举表示 JSON（树形结构）
// =======================================================================

/// 任意 JSON 值：演示枚举构建复杂数据结构。
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    /// Box 包 HashMap 让所有 Json 值等长（HashMap 较大）。
    Object(Box<HashMap<String, Json>>),
}

// =======================================================================
// 泛型枚举：二叉搜索树
// =======================================================================

/// 有序二叉树：演示递归泛型枚举。
pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

/// 二叉树节点。
pub struct TreeNode<T> {
    pub element: T,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    /// 创建空树。
    pub fn new() -> Self {
        BinaryTree::Empty
    }

    /// 插入元素（递归 + 模式匹配）。
    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }));
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }

    /// 中序遍历（升序收集到 Vec）。
    pub fn to_vec(&self) -> Vec<&T> {
        let mut out = Vec::new();
        self.in_order(&mut out);
        out
    }

    fn in_order<'a>(&'a self, out: &mut Vec<&'a T>) {
        match self {
            BinaryTree::Empty => {}
            BinaryTree::NonEmpty(node) => {
                node.left.in_order(out);
                out.push(&node.element);
                node.right.in_order(out);
            }
        }
    }
}

impl<T: Ord> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
