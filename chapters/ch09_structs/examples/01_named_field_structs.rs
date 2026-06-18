//! 9.1 具名字段结构体 + 字段简写 + `..` 展开
//!
//! 关键结论：
//! - `struct Name { field: Type, ... }` 定义具名字段结构体。
//! - 构造用 `Name { field: value, ... }`；当局部变量名与字段同名可省略 value。
//! - 字段访问用 `.field`；自动解引用（穿透 Box/Rc/Arc）。
//! - `..other`：从另一个同类型实例填充未提及的字段。
//! - 字段默认私有；要在外部用 `Name { ... }` 构造需要所有字段都可见。
//!
//! 运行：`cargo run -p ch09_structs --example 01_named_field_structs`

use ch09_structs::{GrayscaleMap, section};

fn main() {
    section("构造具名字段结构体");
    let w = 1024;
    let h = 576;
    let image = GrayscaleMap {
        pixels: vec![0; w * h],
        size: (w, h),
    };
    println!("  size = {:?}", image.size);
    println!("  pixels.len() = {}", image.pixels.len());

    section("字段简写：变量名与字段同名时省略 value");
    let pixels = vec![255; 10];
    let size = (5, 2);
    let image = GrayscaleMap { pixels, size }; // 简写
    println!("  简写后 size = {:?}", image.size);

    section("`..other` 从另一实例填充剩余字段");
    let broom = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };
    let (b1, b2) = chop(broom);
    println!("  b1.name = {}, height = {}", b1.name, b1.height);
    println!("  b2.name = {}, height = {}", b2.name, b2.height);

    section("用关联函数（构造器）构造");
    let m = GrayscaleMap::new((3, 2));
    println!(
        "  GrayscaleMap::new((3,2)) → size = {:?}, pixels = {:?}",
        m.size, m.pixels
    );
}

// === 演示 `..other` 的「砍扫帚」例子 ===

#[derive(Clone)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

#[derive(Clone)]
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

/// 按值接收 Broom，把它「砍成两半」。
fn chop(b: Broom) -> (Broom, Broom) {
    // broom1 从 b 取走大部分字段（包括 String 的所有权）
    let mut broom1 = Broom {
        name: b.name,
        height: b.height / 2,
        health: b.health,
        position: b.position,
        intent: b.intent,
    };
    // broom2 复制 broom1 的所有字段
    let mut broom2 = broom1.clone();
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");
    (broom1, broom2)
}
