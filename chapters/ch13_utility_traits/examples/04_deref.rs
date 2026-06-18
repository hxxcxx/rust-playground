//! 13.4 Deref / DerefMut —— 解引用与自动解引用
//!
//! 关键结论：
//! - `Deref::deref(&self) -> &Target`：让 `*x` 工作，返回内部引用。
//! - `DerefMut::deref_mut(&mut self) -> &mut Target`：让 `*x = v`、`&mut *x` 工作。
//! - 「Deref 强制转换」（deref coercion）：方法调用/传参时，编译器自动链式解引用。
//!   例：`&String` 自动转 `&str`；`&Box<T>` 自动转 `&T`。
//! - 经典用途：
//!   * 智能指针（Box/Rc/Arc/String）让内部类型的方法「直接可用」。
//!   * newtype 模式：包装类型「转发」内部类型的所有方法。
//! - 不要滥用 Deref 模拟「继承」—— 它只适合「智能指针语义」。
//!
//! 运行：`cargo run -p ch13_utility_traits --example 04_deref`

use ch13_utility_traits::{Percent, VecWrapper, section};

fn main() {
    section("VecWrapper：实现 Deref 后能直接调用 Vec 的方法");
    let mut vw = VecWrapper(vec![1, 2, 3]);
    // push 是 Vec 的方法 —— 但通过 Deref 自动解引用，vw.push() 能用！
    vw.push(4);
    vw.push(5);
    // len、iter 也都是 Vec 的方法。
    println!("  vw.len() = {}", vw.len());
    println!("  vw = {:?}", *vw); // 显式 *vw 取出 &Vec
    println!("  vw.iter().sum::<i32>() = {}", vw.iter().sum::<i32>());

    section("DerefMut：可变方法也能转发");
    vw.clear(); // Vec::clear —— 需要可变借用，走 DerefMut
    vw.extend([10, 20]);
    println!("  clear + extend([10,20]) 后 = {:?}", *vw);

    section("Deref 强制转换：&String 自动变 &str");
    let s = String::from("hello");
    // takes_str 只接受 &str，但传 &s 能过 —— 编译器自动 deref。
    takes_str(&s);
    let owned = String::from("world");
    takes_str(&owned);

    section("Box<T> 也是 Deref：解引用到内部 T");
    let b = Box::new(42_i32);
    // *b 解出 i32；方法调用也自动 deref。
    let n = *b;
    println!("  *Box::new(42) = {n}");

    section("链式 Deref：Box<String> → String → str");
    let bs: Box<String> = Box::new(String::from("deep"));
    // bs: Box<String>，但 takes_str(&*bs) 也能简化为 takes_str(&bs)
    // 编译器：&Box<String> → &String → &str（两次 deref 强制转换）
    takes_str(&bs);

    section("Percent newtype + Deref：让 u8 方法可用");
    let p = Percent::new(75);
    // 这里没有为 Percent 实现 Deref —— 演示「可以手写」让 Percent 表现得像 u8。
    // 这里只打印内部值。
    println!("  Percent::new(75) = {p:?}");
    println!("  Percent::new(150).clamp → {:?}", Percent::new(150));

    section("不要用 Deref 模拟继承");
    // Deref 的「自动转发」看起来像继承，但它无法表达「子类额外方法」，
    // 而且会污染命名空间（内部类型的所有方法都冒出来）。
    // 真正想复用行为 → 用 trait + 组合。
    println!("  （Deref 只适合智能指针语义，不是继承的替代品）");
}

/// 只接受 &str 的函数 —— 演示 deref 强制转换。
fn takes_str(s: &str) {
    println!("  takes_str 收到: {s} (长度 {})", s.len());
}
