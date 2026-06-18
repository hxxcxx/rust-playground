//! 第13章 实用特性（Utility Traits）—— 共享类型与示例。
//!
//! 本章核心：标准库里那批「日常开发离不开」的 trait，大多可以 #[derive]。
//!
//! - `Drop`：值离开作用域时自动运行的「析构」—— 释放资源、打印日志、回滚事务。
//! - `Sized`：编译期已知大小的标记 trait；泛型默认要求 `T: Sized`，`?Sized` 可放宽。
//! - `Clone`：深拷贝（`x.clone()`）；`Copy` 是「位拷贝即可」的标记，实现 Copy 后赋值 = 复制。
//! - `Deref` / `DerefMut`：`*x` 解引用 +「自动解引用」（智能指针、newtype 转发）。
//! - `Default`：提供「零值/默认值」（`T::default()`）—— 泛型构造、配置初始化常用。
//! - `AsRef` / `AsMut`：廉价的「借用视图」转换（`&str ↔ &Path` 等）。
//! - `Borrow` / `ToOwned`：更严格的借用关系（HashMap 查询 &str vs String）。
//! - `From` / `Into`：无损转换（`String::from("x")`、`x.into()`）；`TryFrom`/`TryInto` 可能失败。
//! - `Cow`：写时复制 —— 持有借用或拥有，按需克隆（解析器/处理函数常见）。

use std::ops::{Deref, DerefMut};

// =======================================================================
// 工具函数
// =======================================================================

/// 打印带标题的分割线（与前几章风格一致）。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}

// =======================================================================
// 例 1：Drop —— 带日志的资源守卫
// =======================================================================

/// 一个会在 Drop 时打印消息的守卫：演示析构顺序。
#[derive(Debug)]
pub struct Droppy {
    pub name: &'static str,
}

impl Droppy {
    pub fn new(name: &'static str) -> Self {
        println!("  [构造] Droppy({name})");
        Self { name }
    }
}

impl Drop for Droppy {
    fn drop(&mut self) {
        // 离开作用域时自动调用 —— 与声明顺序「相反」（栈式析构）。
        println!("  [析构] Droppy({})", self.name);
    }
}

// =======================================================================
// 例 2：Deref / DerefMut —— newtype「转发」内部类型的方法
// =======================================================================

/// 包装 `Vec<T>` 的类型：实现 Deref 后，Vec 的方法可以直接用。
pub struct VecWrapper<T>(pub Vec<T>);

impl<T> Deref for VecWrapper<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for VecWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// =======================================================================
// 例 3：Default —— 提供默认值
// =======================================================================

/// 一个服务器配置：部分字段用 Default，部分调用方覆盖。
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub timeout_secs: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            max_connections: 100,
            timeout_secs: 30,
        }
    }
}

// =======================================================================
// 例 4：Appellation（书中经典例子）—— Clone/Copy/Display
// =======================================================================

/// 名字：演示 Copy 类型 + Display。
/// 注意：String 不能 Copy（堆分配），所以这里用 &'static str 才能 Copy。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Appellation {
    pub name: &'static str,
    pub nick: &'static str,
}

impl std::fmt::Display for Appellation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.nick)
    }
}

// =======================================================================
// 例 5：Bounded —— 演示限制范围的 newtype + Deref
// =======================================================================

/// 一个被限制在 [0, 100] 的百分比：演示「封装 + 校验」。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Percent(pub u8);

impl Percent {
    /// 构造并自动 clamp 到 [0, 100]。
    pub fn new(value: i32) -> Self {
        Percent(value.clamp(0, 100) as u8)
    }
}
