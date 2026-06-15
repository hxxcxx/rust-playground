//! 3.5 元组：固定长度、可异构、用 .0/.1 访问或模式匹配
//!
//! 运行：`cargo run -p ch03_basic_types --example 05_tuple`

use ch03_basic_types::section;

fn main() {
    section("元组字面量");
    let pair: (&str, i32) = ("Brazil", 1985);
    println!("pair.0 = {}, pair.1 = {}", pair.0, pair.1);

    section("模式匹配解构（推荐）");
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
    println!("head = {head:?}\ntail = {tail:?}");

    section("函数多返回值");
    let (q, r) = divmod(17, 5);
    assert_eq!((q, r), (3, 2));
    println!("17 / 5 = {q} 余 {r}");

    section("单元类型 ()：唯一值为 ()，表示没有有效信息");
    let unit: () = ();
    println!("size_of::<()>() = {} byte", size_of_val(&unit));

    section("用作轻量结构体（如二维尺寸）");
    let bounds: (usize, usize) = (1920, 1080);
    println!("width = {}, height = {}", bounds.0, bounds.1);

    section("尾随逗号合法");
    let _ok: (&str, i32) = ("trailing", 1);
    let _single: (&str,) = ("lonely",); // 单元素元组，逗号必须有
    println!("单元素元组：类型是 (&str,)，而非 &str");

    section("嵌套元组与一次性解构");
    let ((a, b), c) = ((1, 2), 3);
    println!("a={a}, b={b}, c={c}");

    section("交换两个变量：std::mem::swap 用泛型元组语义");
    let (mut x, mut y) = (1, 2);
    std::mem::swap(&mut x, &mut y);
    assert_eq!((x, y), (2, 1));
    println!("swap 后: x={x}, y={y}");
}

/// 同时返回商和余数：典型元组返回场景。
fn divmod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}
