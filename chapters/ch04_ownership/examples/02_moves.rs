//! 4.2 移动 (Moves)
//!
//! 关键结论：
//! - 对大多数类型，赋值 / 传参 / 返回值不是复制，而是「移动」：
//!   源变量变为未初始化，目标变量取得所有权。
//! - 移动只搬运「值本身」（Vec/String 只是三个字的头部），堆缓冲区位置不变。
//! - 移动让你像 Python 一样廉价赋值，又像 C++ 一样保持所有权清晰（无需 GC/引用计数）。
//! - 想要深拷贝必须显式调用 `.clone()`。
//!
//! 运行：`cargo run -p ch04_ownership --example 02_moves`

use ch04_ownership::{Person, section};

/// 演示：`let t = s;` 会把 `s` 移动到 `t`，`s` 之后不可再用。
#[allow(dead_code)]
fn move_on_let() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s; // s 移动到 t：三字头部被复制到 t，s 变为未初始化
    // let u = s;  // ❌ 编译错误：use of moved value `s`（E0382）
    let _u = t.clone(); // 想要独立副本，必须显式 clone（深拷贝）
    println!("t = {_u:?}");
}

/// 演示：给「已初始化」的变量赋新值，会先丢弃旧值。
#[allow(dead_code)]
fn move_on_assignment_drops_old() {
    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string(); // 此处丢弃旧值 "Govinda"（其堆缓冲区被释放）
    println!("s = {s}");
}

/// 演示：如果原值已被移动走，再次赋值时不会触发 drop（因为变量已未初始化）。
#[allow(dead_code)]
fn move_then_assign_no_drop() {
    let mut s = "Govinda".to_string();
    let t = s; // t 接管所有权，s 现在未初始化
    s = "Siddhartha".to_string(); // s 当前未初始化，所以不会丢弃任何东西
    println!("s = {s}, t = {t}");
}

/// 演示：函数返回值是 move —— 调用者获得所有权。
fn make_greeting(name: &str) -> String {
    // 返回的 String 的所有权从 make_greeting 内部移动到调用者
    format!("Hello, {name}!")
}

/// 演示：把值传给函数也是 move —— 函数参数获得所有权。
fn consume(s: String) {
    println!("consumed: {s}");
} // s 在此被 drop

/// 演示：构造结构体时，字段值会移动进结构体。
/// 演示：构造结构体时，字段值会移动进结构体。
#[allow(clippy::vec_init_then_push)]
fn build_and_push() {
    // 这里发生了多次 move：
    //   1) to_string() 的返回值移动进 Person.name 字段；
    //   2) 整个 Person 移动进 Vec::push 的参数；
    //   3) Vec 接管 Person，间接也接管了 name 中的 String。
    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    println!("composers = {:?}", composers[0].name);
}

fn main() {
    section("`let t = s` 是移动：s 变为未初始化");
    move_on_let();

    section("赋值会丢弃旧值");
    move_on_assignment_drops_old();

    section("若旧值已被移动走，再赋值不会触发 drop");
    move_then_assign_no_drop();

    section("更多 move 场景：函数返回值、传参、构造结构体");
    let g = make_greeting("world"); // 返回值 move 给 g
    println!("{g}");
    consume(g); // g move 给 consume 的参数；之后 g 不可再用
    build_and_push();

    section("总结");
    println!("几乎所有使用值的地方都是 move；想要深拷贝必须显式 .clone()。");
}
