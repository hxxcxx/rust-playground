//! 11.1 trait object —— 动态分发
//!
//! 关键结论：
//! - trait object 是「trait 类型的值」，背后是「具体类型 + 指向该类型方法表的指针（vtable）」。
//! - 书写：`&dyn Trait`、`Box<dyn Trait>`、`Rc<dyn Trait>`（必须用指针，因为大小未知）。
//! - 动态分发：调用方法时，运行期查 vtable 找到具体实现 —— 一个容器里能装「不同」类型。
//! - 代价：多一次指针解引用 + 间接调用（不利内联）；对象本身需要堆分配（Box 时）。
//! - 限制：trait 必须「对象安全」（object-safe）：
//!   * 方法不能返回 Self；
//!   * 方法不能有泛型参数；
//!   * 不能用关联类型作为「方法的接收者以外的位置」（实际更细致，详见标准库文档）。
//!
//! 运行：`cargo run -p ch11_traits_and_generics --example 01_trait_objects`

use ch11_traits_and_generics::{Circle, IsShape, Rectangle, section};

/// 接收 trait object（引用形式）：函数可以处理「任何 IsShape」。
fn print_area(shape: &dyn IsShape) {
    // 动态分发：这里 area() 到底调用谁的，运行期才决定。
    println!("  {} 的面积 = {:.2}", shape.name(), shape.area());
}

/// 接收 trait object（Box 形式）：拿到所有权，可以塞进集合长期保存。
fn largest_area(shapes: &[Box<dyn IsShape>]) -> f64 {
    // 借助 IsShape 提供的默认实现以外的 area()。
    shapes
        .iter()
        .map(|s| s.area())
        .fold(f64::NEG_INFINITY, f64::max)
}

fn main() {
    section("用 &dyn Trait 调用（动态分发）");
    let c = Circle { radius: 2.0 };
    let r = Rectangle { width: 3.0, height: 4.0 };
    // 同一个函数，传「不同」的具体类型 —— 这就是多态。
    print_area(&c);
    print_area(&r);

    section("Vec<Box<dyn Trait>>：异构集合");
    // 一个 Vec 里同时装 Circle 和 Rectangle —— 这正是泛型做不到、trait object 做到的。
    let shapes: Vec<Box<dyn IsShape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle { width: 2.0, height: 5.0 }),
        Box::new(Circle { radius: 3.0 }),
    ];
    for s in &shapes {
        println!("  - {} 面积 = {:.2}", s.name(), s.area());
    }
    println!("  最大面积 = {:.2}", largest_area(&shapes));

    section("trait object 的类型：dyn IsShape 是「unsized」");
    // dyn IsShape 的大小编译期未知（不同实现者大小不同），
    // 所以必须放在指针后面：&dyn / Box<dyn> / Rc<dyn>。
    // 下面这行会编译失败（unsized 不能直接当变量）：
    //   let s: dyn IsShape = c;
    // 正确写法是用引用或 Box 包起来：
    let s: &dyn IsShape = &c;
    println!("  &dyn IsShape 的 name = {}", s.name());
    println!(
        "  size_of::<&dyn IsShape>() = {} 字节（胖指针：数据指针 + vtable 指针）",
        std::mem::size_of::<&dyn IsShape>()
    );
    println!(
        "  size_of::<Box<dyn IsShape>>() = {} 字节",
        std::mem::size_of::<Box<dyn IsShape>>()
    );

    section("对象安全的限制：trait 方法不能有泛型参数 / 返回 Self");
    // 例如 Clone 的 clone(&self) -> Self 返回 Self，因此 Clone 不是对象安全的，
    // 不能写 Box<dyn Clone>（除非用 Clone 作为约束，而非对象类型）。
    println!("  （IsShape 所有方法都没问题，所以可以做 trait object）");
}
