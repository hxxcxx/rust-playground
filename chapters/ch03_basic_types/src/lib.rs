//! 第3章 基本类型 —— 共享工具。

/// 打印带标题的分割线，便于在 example 输出中区分小节。
pub fn section(title: impl AsRef<str>) {
    let title = title.as_ref();
    println!("\n=== {title} ===");
}
