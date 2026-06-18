//! 11.2 定义与实现 trait —— 默认方法、完全限定、impl 与 Display
//!
//! 关键结论：
//! - `trait Name { fn method(&self) -> ReturnType; }` 定义一个方法签名。
//! - `impl Name for Type { ... }` 为具体类型实现 trait。
//! - trait 里可以给方法提供「默认实现」（default method），实现者可选择覆盖。
//! - 一个类型可以实现「很多」trait（多个 impl 块各管一个）。
//! - 也可以为 trait 写默认实现，让它「调用同一个 trait 的其它方法」
//!   （实现者只要实现一个核心方法，剩下的自动获得）。
//!
//! 运行：`cargo run -p ch11_traits_and_generics --example 02_defining_traits`

use ch11_traits_and_generics::{Circle, IsShape, Rectangle, section};

/// 自定义 trait：带「默认方法 + 互相调用」的设计。
pub trait Describable {
    /// 必须实现：返回核心描述。
    fn describe(&self) -> String;

    /// 默认方法：基于 describe() 给出「带前缀」的描述，可覆盖。
    fn describe_loudly(&self) -> String {
        // 注意：这里调用的 describe() 是 trait 自身的方法，
        // 实现者实现了 describe()，这里就自动用上。
        format!(">>> {} <<<", self.describe())
    }

    /// 默认方法：返回描述的长度。
    fn describe_len(&self) -> usize {
        self.describe().len()
    }
}

/// 为 Circle 实现 Describable（只实现必需方法，其余用默认）。
impl Describable for Circle {
    fn describe(&self) -> String {
        format!("circle r={}", self.radius)
    }
}

/// 为 Rectangle 实现 Describable，并覆盖 describe_loudly。
impl Describable for Rectangle {
    fn describe(&self) -> String {
        format!("rect {}x{}", self.width, self.height)
    }
    fn describe_loudly(&self) -> String {
        // 覆盖默认实现，走自己的逻辑。
        format!("[ {} x {} ]", self.width, self.height)
    }
}

/// 演示「一个类型可以实现多个 trait」。
pub trait HasLabel {
    fn label(&self) -> &str;
}

impl HasLabel for Circle {
    fn label(&self) -> &str {
        "CIRCLE"
    }
}

impl HasLabel for Rectangle {
    fn label(&self) -> &str {
        "RECT"
    }
}

fn main() {
    section("实现 trait 的必需方法");
    let c = Circle { radius: 2.5 };
    let r = Rectangle { width: 3.0, height: 4.0 };
    println!("  Circle.describe() = {}", c.describe());
    println!("  Rectangle.describe() = {}", r.describe());

    section("默认方法：自动获得 describe_loudly / describe_len");
    // Circle 没实现 describe_loudly，用的是 trait 的默认实现。
    println!("  Circle.describe_loudly() = {}", c.describe_loudly());
    println!("  Circle.describe_len()    = {}", c.describe_len());

    section("覆盖默认方法");
    // Rectangle 覆盖了 describe_loudly，行为不同。
    println!("  Rectangle.describe_loudly() = {}", r.describe_loudly());

    section("一个类型可以实现多个 trait");
    println!("  Circle.label()    = {}", c.label());
    println!("  Rectangle.label() = {}", r.label());

    section("组合多个 trait 一起用（trait bound + 方法调用）");
    show_describable(&c);
    show_describable(&r);

    section("trait 方法也出现在自动补全里 —— IsShape::area / name");
    println!("  Circle  IsShape: name={}, area={:.2}", c.name(), c.area());
    println!("  Rect    IsShape: name={}, area={:.2}", r.name(), r.area());
}

/// 接收「同时实现了 Describable 和 HasLabel」的引用。
fn show_describable<T: Describable + HasLabel>(item: &T) {
    println!(
        "  [{}] {} (len={})",
        item.label(),
        item.describe_loudly(),
        item.describe_len()
    );
}
