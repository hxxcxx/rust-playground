//! 12.2 一元运算符 Neg / Not + 复合赋值 AddAssign / SubAssign 等
//!
//! 关键结论：
//! - 一元运算符：`-x`（Neg）、`!x`（Not）—— 也都是 trait。
//! - 复合赋值：`+= -= *= /= %= &= |= ^= <<= >>=` 各对应一个 `XxxAssign` trait。
//! - 复合赋值「不是」从普通运算符自动派生的，必须单独实现（接收 &mut self）。
//! - 好处：`a += b` 比 `a = a + b` 少一次移动/拷贝 —— 对大类型更高效。
//!
//! 运行：`cargo run -p ch12_operator_overloading --example 02_unary_assign`

use ch12_operator_overloading::{Complex, section};
use std::ops::{AddAssign, MulAssign, Neg, Not, SubAssign};

fn main() {
    section("一元负：-c 调用 Neg::neg");
    let a = Complex::new(1.0, 2.0);
    let neg_a = -a; // 等价于 Neg::neg(a)
    println!("  -({a:?}) = {neg_a:?}");

    section("一元非：!x 调用 Not::not（布尔取反 / 整数按位取反）");
    let b = false;
    println!("  !false = {}", !b);
    let n: u8 = 0b1010_1010;
    println!("  !0b1010_1010 (u8) = {:#010b}", !n);

    section("复合赋值：c1 += c2（必须单独 impl AddAssign）");
    let mut c1 = Complex::new(1.0, 1.0);
    let c2 = Complex::new(2.0, 3.0);
    c1 += c2; // 等价于 AddAssign::add_assign(&mut c1, c2)
    println!("  c1 += c2 后 = {c1:?}");

    section("复合赋值的性能优势：就地修改");
    let mut acc = Complex::new(0.0, 0.0);
    for k in 0..5 {
        // += 不创建中间 Complex，直接累加到 acc 的字段上。
        acc += Complex::new(k as f64, k as f64);
    }
    println!("  累加 0..5 后 = {acc:?}");

    section("Counter：演示 += -= *= 三种复合赋值");
    let mut cnt = Counter(10);
    cnt += Counter(5);
    println!("  10 += 5  → {cnt:?}");
    cnt -= Counter(3);
    println!("  15 -= 3  → {cnt:?}");
    cnt *= Counter(2);
    println!("  12 *= 2  → {cnt:?}");
}

/// 一个简单计数器：演示复合赋值运算符。
#[derive(Debug, Clone, Copy)]
struct Counter(i32);

impl AddAssign for Counter {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Counter {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl MulAssign for Counter {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Neg for Counter {
    type Output = Self;
    fn neg(self) -> Self {
        Counter(-self.0)
    }
}
