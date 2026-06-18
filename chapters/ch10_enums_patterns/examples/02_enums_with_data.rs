//! 10.2 含数据的枚举变体
//!
//! 关键结论：
//! - 三种变体形式（对应三种 struct）：
//!   1. 无数据（类单元变体）：`JustNow`
//!   2. 元组变体：`InThePast(TimeUnit, u32)`，构造像函数调用
//!   3. 结构体变体：`Shape::Sphere { center, radius }`，带字段名
//! - 内存：枚举 = 1 个小整数标签 + 足以容纳「最大变体」所有字段的内存。
//! - 用枚举表示 JSON 这种「多形态」数据非常简洁（serde_json 就是这样）。
//! - `Box<HashMap>` 可让大字段变成指针，让枚举更紧凑。
//!
//! 运行：`cargo run -p ch10_enums_patterns --example 02_enums_with_data`

use ch10_enums_patterns::{Json, RoughTime, TimeUnit, section};
use std::collections::HashMap;

fn main() {
    section("元组变体：带位置参数");
    let past = RoughTime::InThePast(TimeUnit::Years, 7);
    let future = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    let now = RoughTime::JustNow;
    println!("  past   = {past:?}");
    println!("  future = {future:?}");
    println!("  now    = {now:?}");

    section("用 match 安全取出变体里的数据");
    println!("  → {}", rough_time_to_english(past));
    println!("  → {}", rough_time_to_english(future));
    println!("  → {}", rough_time_to_english(now));

    section("结构体变体（带字段名）");
    let unit_sphere = Shape::Sphere {
        center: Point3d::origin(),
        radius: 1.0,
    };
    println!("  {:?} 体积 = {:.3}", unit_sphere, unit_sphere.volume());

    section("三种变体可以共存于一个枚举");
    let _status = RelationshipStatus::ItsComplicated(Some("secret".into()));
    println!("  RelationshipStatus 同时有 3 种变体形式");

    section("用枚举表示任意 JSON：树形数据结构");
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), Json::String("Alice".into()));
    obj.insert("age".to_string(), Json::Number(30.0));
    obj.insert(
        "tags".to_string(),
        Json::Array(vec![
            Json::String("rust".into()),
            Json::Boolean(true),
            Json::Null,
        ]),
    );
    let json = Json::Object(Box::new(obj));
    println!("  {:?}", json);

    section("Box<HashMap> 让 Json 等长（4 个机器字）");
    println!("  size_of::<Json>() = {} 字节", std::mem::size_of::<Json>());
}

/// 把 RoughTime 翻译成英文 —— 用 match 提取变体里的数据。
fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => format!("{count} {} ago", units.plural()),
        RoughTime::JustNow => "just now".to_string(),
        RoughTime::InTheFuture(units, count) => format!("{count} {} from now", units.plural()),
    }
}

// === 演示结构体变体 ===

#[derive(Debug)]
struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3d {
    fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[derive(Debug)]
enum Shape {
    Sphere { center: Point3d, radius: f64 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}

impl Shape {
    fn volume(&self) -> f64 {
        match self {
            Shape::Sphere { radius, .. } => {
                (4.0 / 3.0) * std::f64::consts::PI * radius * radius * radius
            }
            Shape::Cuboid { corner1, corner2 } => {
                (corner2.x - corner1.x).abs()
                    * (corner2.y - corner1.y).abs()
                    * (corner2.z - corner1.z).abs()
            }
        }
    }
}

/// 演示同一枚举内「3 种变体形式」共存。
#[allow(dead_code)]
enum RelationshipStatus {
    Single,                                                         // 类单元变体
    InARelationship,                                                // 类单元变体
    ItsComplicated(Option<String>),                                 // 元组变体
    ItsExtremelyComplicated { reason: String, duration_days: u32 }, // 结构体变体
}
