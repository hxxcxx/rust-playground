//! 10.3 泛型枚举：Option / Result / BinaryTree
//!
//! 关键结论：
//! - `enum Option<T> { None, Some(T) }` —— 标准库最常用的泛型枚举。
//! - `enum Result<T, E> { Ok(T), Err(E) }` —— 错误处理的基石。
//! - 递归泛型枚举可以表示树：`BinaryTree<T>` 内含 `Box<TreeNode<T>>`。
//! - `Option<Box<T>>` 在内存中只有 1 个机器字：用 0 表示 None，非零指针表示 Some。
//! - 二叉搜索树插入：递归 + match，是 enum + Box + 模式的经典组合。
//!
//! 运行：`cargo run -p ch10_enums_patterns --example 03_generic_enums`

use ch10_enums_patterns::{BinaryTree, section};

fn main() {
    section("Option<T>：表示「可能没有值」");
    let some: Option<i32> = Some(42);
    let none: Option<i32> = None;
    println!("  some = {some:?}, none = {none:?}");

    section("Result<T, E>：表示「可能失败」");
    let ok: Result<i32, &str> = Ok(42);
    let err: Result<i32, &str> = Err("出错了");
    println!("  ok = {ok:?}, err = {err:?}");

    section("Option<Box<T>> 内存优化：用 null 表示 None");
    let some_boxed: Option<Box<i32>> = Some(Box::new(42));
    let none_boxed: Option<Box<i32>> = None;
    println!(
        "  size_of::<Option<Box<i32>>>() = {} 字节（指针大小）",
        std::mem::size_of::<Option<Box<i32>>>()
    );
    println!("  some = {some_boxed:?}");
    println!("  none = {none_boxed:?}");

    section("递归泛型枚举：二叉搜索树");
    let mut tree = BinaryTree::<&str>::new();
    for planet in ["Saturn", "Mars", "Jupiter", "Mercury", "Uranus", "Venus"] {
        tree.add(planet);
    }
    println!("  中序遍历（升序）= {:?}", tree.to_vec());

    section("数字二叉树");
    let mut nums = BinaryTree::<i32>::new();
    for n in [50, 30, 70, 20, 40, 60, 80, 10] {
        nums.add(n);
    }
    println!("  升序 = {:?}", nums.to_vec());

    section("Option 的常用方法");
    // 用变量代替字面量，避免 clippy 把 unwrap_or 当 literal unwrap 静态求值
    let x: Option<i32> = make_some();
    let none: Option<i32> = make_none();
    println!("  Some(5).map(|v| v * 2) = {:?}", x.map(|v| v * 2));
    // map 比 and_then(|v| Some(y)) 更合适，这里演示 and_then 的另一种用法
    println!("  Some(5).and_then(non_zero) = {:?}", x.and_then(non_zero));
    println!("  Some(5).unwrap_or(0) = {}", x.unwrap_or(0));
    println!("  None.unwrap_or(0) = {}", none.unwrap_or(0));

    section("空树也是合法的 BinaryTree");
    let empty: BinaryTree<i32> = BinaryTree::Empty;
    println!("  空树.to_vec() = {:?}", empty.to_vec());
}

fn make_five() -> i32 {
    5
}

fn make_some() -> Option<i32> {
    Some(make_five())
}

fn make_none<T>() -> Option<T> {
    None
}

/// 演示 and_then：返回 Option 的转换函数。
fn non_zero(n: i32) -> Option<i32> {
    if n != 0 { Some(n) } else { None }
}
