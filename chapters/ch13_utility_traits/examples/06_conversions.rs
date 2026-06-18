//! 13.6 类型转换 —— From/Into、AsRef/AsMut、Borrow、TryFrom/TryInto、Cow
//!
//! 关键结论：
//! - `From<T>`：从 T 转换到 Self；`Into<T>`：把 self 转成 T。
//!   实现 `From<T> for U` 自动获得 `Into<U> for T`（反之不然）。
//!   习惯：实现 From，调用方用 into()（更顺手）。
//! - `AsRef<T>` / `AsMut<T>`：廉价借用转换（不消耗所有权），返回 &T / &mut T。
//! - `Borrow<T>`：更严格的「借用」——要求 Hash/Eq/Ord 与原类型一致（HashMap 查询用）。
//! - `TryFrom` / `TryInto`：可能失败的转换，返回 `Result<T, Error>`。
//! - `Cow<T>`（写时复制）：要么借用 &T，要么拥有 T 的 Owned 形式；
//!   读取时零开销，需要修改时才克隆 —— 解析器/文本处理常见。
//!
//! 运行：`cargo run -p ch13_utility_traits --example 06_conversions`

use ch13_utility_traits::section;
use std::borrow::Cow;

fn main() {
    section("From / Into：无损转换");
    // 实现 From 自动获得 Into。
    let s: String = String::from("hello"); // From<&str>
    let s2: String = "world".into(); // Into —— 等价
    println!("  from: {s}, into: {s2}");

    section("自定义 From：让类型转换更自然");
    let age = Age::from(30_u32);
    let age2: Age = 25_u32.into();
    println!("  {age:?}, {age2:?}");

    section("函数参数用 Into 减少调用方负担");
    // 函数签名 fn greet(name: impl Into<String>)
    // 调用方既能传 &str 也能传 String，不用自己转换。
    greet("Alice");
    greet(String::from("Bob"));

    section("AsRef：廉价借用视图");
    // 接受任何「能 as_ref 到 &str」的东西 —— String、&str、PathBuf... 都行。
    fn print_len(s: impl AsRef<str>) {
        println!("  长度 = {}", s.as_ref().len());
    }
    print_len("hello");
    print_len(String::from("world"));

    section("Borrow：HashMap 用 &str 查 String 键");
    use std::collections::HashMap;
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("alice".to_string(), 1);
    // 不用 Borrow 的话，查询要构造 String（分配）；
    // HashMap 借助 Borrow<str>，可以直接用 &str 查。
    if let Some(v) = map.get("alice") {
        println!("  map.get(\"alice\") = {v}");
    }

    section("TryFrom / TryInto：可能失败的转换");
    // u32 → u8：值超出 255 会失败。
    let small: u8 = 200u32.try_into().unwrap();
    println!("  200u32.try_into::<u8>() = {small}");
    let big: Result<u8, _> = 300u32.try_into();
    println!("  300u32.try_into::<u8>() = {:?}", big); // Err

    section("自定义 TryFrom：带校验");
    let ok = NonZero::try_from(42).unwrap();
    println!("  NonZero::try_from(42) = {ok:?}");
    let err = NonZero::try_from(0);
    println!("  NonZero::try_from(0)  = {:?}", err); // Err("zero is not allowed")

    section("Cow：写时复制");
    let input = "hello world";
    let owned_input = String::from("HELLO WORLD");

    // process 既可能借用输入，也可能（如果需要修改）克隆一份。
    let borrowed = process(input); // 不需要修改 → 借用，零拷贝
    let modified = process(&owned_input); // 需要修改 → 克隆
    println!("  借用结果（is_borrowed）: {}", matches!(borrowed, Cow::Borrowed(_)));
    println!("  修改结果（is_owned）  : {}", matches!(modified, Cow::Owned(_)));
    println!("  借用: {borrowed}");
    println!("  修改: {modified}");

    section("Cow 的典型场景：返回 &str 或 String 不用统一类型");
    // 一个函数「可能」返回引用、「可能」返回新字符串 —— Cow 让两者统一。
    let normalized = normalize("  Spaced  Text  ");
    println!("  normalize(\"  Spaced  Text  \") = {normalized}");
    let already = normalize("clean");
    println!("  normalize(\"clean\") = {already} (无需克隆 → Borrowed)");
}

/// 年龄：演示自定义 From。
#[derive(Debug)]
struct Age(u32);

impl From<u32> for Age {
    fn from(value: u32) -> Self {
        Age(value)
    }
}

/// 接受任何能 Into<String> 的参数。
fn greet(name: impl Into<String>) {
    let n: String = name.into();
    println!("  你好, {n}!");
}

/// 非零整数：演示 TryFrom。
#[derive(Debug)]
struct NonZero(u32);

impl TryFrom<u32> for NonZero {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err("zero is not allowed")
        } else {
            Ok(NonZero(value))
        }
    }
}

/// 演示 Cow：如果输入含大写字母就克隆并改写，否则直接借用。
fn process(input: &str) -> Cow<'_, str> {
    if input.chars().any(|c| c.is_uppercase()) {
        // 需要修改 → 克隆一份，转小写。
        Cow::Owned(input.to_lowercase())
    } else {
        // 不需要修改 → 零拷贝，直接借用。
        Cow::Borrowed(input)
    }
}

/// 规范化空格：trim + 压缩多空格。
/// 输入已经干净时直接借用，否则克隆修改。
fn normalize(input: &str) -> Cow<'_, str> {
    let trimmed = input.trim();
    if trimmed.contains("  ") {
        // 有连续空格 → 需要构造新字符串。
        let collapsed: String = trimmed.split_whitespace().collect::<Vec<_>>().join(" ");
        Cow::Owned(collapsed)
    } else if trimmed.len() == input.len() {
        // 原样可用 → 借用。
        Cow::Borrowed(input)
    } else {
        // 只是 trim 了 → 借用 trim 后的切片。
        Cow::Borrowed(trimmed)
    }
}
