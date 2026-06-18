//! 11.3 孤儿规则、超特性、完全限定方法调用
//!
//! 关键结论：
//! - 孤儿规则（Orphan Rule）：要想 `impl SomeTrait for SomeType` 通过，
//!   `SomeTrait` 或 `SomeType` 至少有一个是在「当前 crate」定义的。
//!   好处：避免两个 crate 对同一对 (Trait, Type) 重复实现造成冲突。
//! - 超特性（supertrait）：`trait Sub: Super { ... }` 表示实现 Sub 必须先实现 Super。
//! - 完全限定方法调用（Fully Qualified Syntax）：
//!   `<Type as Trait>::method(...)` 用来消歧义 —— 当多个 trait 有同名方法时。
//! - Rust 不支持「继承」字段，但可以用 supertrait 复用「行为」。
//!
//! 运行：`cargo run -p ch11_traits_and_generics --example 03_orphan_and_supertraits`

use ch11_traits_and_generics::{IsClickable, IsVisible, NewsArticle, Summary, section};

// =====================================================================
// 孤儿规则演示
// =====================================================================
//
// 孤儿规则（Orphan Rule）：`impl Trait for Type` 只有当
// `Trait` 或 `Type` 中至少一个在「当前 crate」定义时才合法。
//
// 注意：i32 这样的「原始类型」有更严格的限制——
// 即使 trait 在本 crate 里，对原始类型的 impl 也必须写在「定义 trait 的 crate」中，
// 不能写在 example / 子 crate 里。所以我们在 src/lib.rs 中写了：
//     impl Summary for i32 { ... }
// 这里直接复用即可。
//
// ❌ 不合法的例子（仅作说明，不编译）：
//   impl std::fmt::Display for i32 { ... }
// 原因：Display 和 i32 都定义在 std 里，都不是本 crate 的 —— 违反孤儿规则。
// 想给外来类型加 Display？只能用「newtype 包装」（见下文）。

/// newtype 模式：包一层本地类型，从而可以「绕过」孤儿规则。
pub struct MyI32(pub i32);

// 现在 MyI32 是本 crate 的，所以可以给它实现任意 trait，包括 std 的 Display。
impl std::fmt::Display for MyI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyI32({})", self.0)
    }
}

// =====================================================================
// 超特性：IsClickable: IsVisible
// =====================================================================

/// 一个按钮：实现了 IsVisible（必须先实现），才能实现 IsClickable。
#[derive(Debug)]
pub struct Button {
    pub label: String,
}

// 第一步：实现超特性 IsVisible。
impl IsVisible for Button {
    fn draw(&self) -> String {
        format!("[Button: {}]", self.label)
    }
}

// 第二步：现在才可以实现子特性 IsClickable。
impl IsClickable for Button {
    fn on_click(&self) -> &str {
        "button-clicked"
    }
}

/// 一张图片：只实现 IsVisible，不实现 IsClickable。
pub struct Picture {
    pub path: String,
}

impl IsVisible for Picture {
    fn draw(&self) -> String {
        format!("<img src={}>", self.path)
    }
}

// =====================================================================
// 同名方法歧义 → 完全限定语法
// =====================================================================

/// trait A 和 trait B 都有 foo()，类型同时实现两者时该用哪个？
pub trait Pilot {
    fn fly(&self) -> String;
}

pub trait Wizard {
    fn fly(&self) -> String;
}

#[derive(Debug)]
pub struct Human;

impl Human {
    // Human 自身的 fly（impl 块里直接定义的方法）
    fn fly(&self) -> String {
        "human waving arms (not flying)".into()
    }
}

impl Pilot for Human {
    fn fly(&self) -> String {
        "pilot flying an airplane".into()
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        "wizard on a broomstick".into()
    }
}

fn main() {
    section("孤儿规则：lib crate 已为 i32 实现 Summary（原始类型必须写在 trait 所在 crate）");
    let n: i32 = 42;
    println!("  42.summarize() = {}", n.summarize());
    let news = NewsArticle {
        headline: "Rust 1.85 发布".into(),
        location: "上海".into(),
    };
    println!("  news.summarize() = {}", news.summarize());

    section("newtype 模式绕过孤儿规则（为 MyI32 实现 Display）");
    let mine = MyI32(7);
    println!("  {}", mine);

    section("超特性：实现 IsClickable 必须先实现 IsVisible");
    let btn = Button { label: "OK".into() };
    println!("  IsVisible::draw  = {}", btn.draw());
    println!("  IsClickable::on_click = {}", btn.on_click());

    let pic = Picture { path: "a.png".into() };
    println!("  pic.draw() = {}", pic.draw());

    section("同名方法：默认调用「本类型自己」的方法");
    let h = Human;
    // 直接调用 —— 默认走 Human 自己的 fly。
    println!("  h.fly()                  = {}", h.fly());

    section("完全限定方法调用：<Type as Trait>::method(...)");
    println!("  <Human as Pilot>::fly()  = {}", <Human as Pilot>::fly(&h));
    println!("  <Human as Wizard>::fly() = {}", <Human as Wizard>::fly(&h));

    section("完全限定也可以用于消歧 trait 方法 vs 固有方法");
    // 即使是 trait 没有歧义，完全限定写法也合法（只是没必要）。
    println!("  Human::fly(&h)           = {}", Human::fly(&h));
}
