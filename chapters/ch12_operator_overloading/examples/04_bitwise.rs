//! 12.4 位运算符：BitAnd / BitOr / BitXor / Not / Shl / Shr
//!
//! 关键结论：
//! - 位运算符也对应 trait（`std::ops`）：
//!   `&` BitAnd / `|` BitOr / `^` BitXor / `!` Not
//!   `<<` Shl / `>>` Shr。
//! - 整数类型默认就实现了这些；自定义类型可以「语义化」位运算。
//! - 常见用法：位掩码集合（flags），把 `|` 当「并集」、`&` 当「交集」。
//! - 注意 `<<` / `>>` 的右操作数（移位量）类型可以与左操作数不同（Rhs 泛型）。
//!
//! 运行：`cargo run -p ch12_operator_overloading --example 04_bitwise`

use ch12_operator_overloading::{Bitmask, section};

fn main() {
    section("Bitmask 位掩码：把 | & ^ ! 用作集合运算");
    let a = Bitmask(0b1100_0011);
    let b = Bitmask(0b1010_0101);
    println!("  a       = {:#010b}", a.0);
    println!("  b       = {:#010b}", b.0);
    // | → 并集（任一为 1 即为 1）
    println!("  a | b   = {:#010b}", (a | b).0);
    // & → 交集（都为 1 才为 1）
    println!("  a & b   = {:#010b}", (a & b).0);
    // ^ → 对称差（不同为 1）
    println!("  a ^ b   = {:#010b}", (a ^ b).0);
    // ! → 取反
    println!("  !a      = {:#010b}", (!a).0);

    section("移位运算 Shl / Shr");
    let n: u32 = 0b0001;
    println!("  1 << 4  = {:#010b} (= {})", n << 4, n << 4);
    println!("  0b10000_0000 >> 4 = {:#010b}", 0b1_0000_0000_u32 >> 4);

    section("实战：权限标志位");
    let read = Permission::READ;
    let write = Permission::WRITE;
    let exec = Permission::EXECUTE;
    // 用 | 组合权限（这是「位运算符重载」最经典的用法 —— C 时代的 flags）。
    let rw = read | write;
    println!("  read | write = {rw:?}");
    // 用 & 检查是否包含某权限。
    println!("  rw 包含 write? {}", rw.contains(write));
    println!("  rw 包含 exec?  {}", rw.contains(exec));

    section("整数上 & | ^ ! 本来就能用（标准库已实现）");
    let x: u8 = 0xF0;
    let y: u8 = 0x0F;
    println!("  0xF0 & 0x0F = {:#04x}", x & y); // 0
    println!("  0xF0 | 0x0F = {:#04x}", x | y); // FF
    println!("  0xFF ^ 0x0F = {:#04x}", 0xFFu8 ^ 0x0F); // F0
}

/// 权限标志：用 BitOr 组合、BitAnd 测试。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Permission(u8);

impl Permission {
    const READ_BITS: u8 = 0b001;
    const WRITE_BITS: u8 = 0b010;
    const EXEC_BITS: u8 = 0b100;

    const READ: Permission = Permission(Self::READ_BITS);
    const WRITE: Permission = Permission(Self::WRITE_BITS);
    const EXECUTE: Permission = Permission(Self::EXEC_BITS);

    /// 判断是否「包含」另一个权限（子集测试）。
    fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for Permission {
    type Output = Permission;
    fn bitor(self, rhs: Self) -> Permission {
        Permission(self.0 | rhs.0)
    }
}
