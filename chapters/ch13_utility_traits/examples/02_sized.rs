//! 13.2 Sized —— 编译期已知大小
//!
//! 关键结论：
//! - `Sized` 是「标记 trait」：表示类型在「编译期」大小已知。
//! - 所有「固定大小」类型（i32、结构体、数组...）自动实现 Sized。
//! - 切片 `[T]`、`str`、`dyn Trait` 等大小未知（unsized / DST），不实现 Sized。
//! - 泛型默认 `T: Sized`；用 `T: ?Sized` 放宽，允许接收「胖指针」。
//! - 胖指针（&[T]、&str、&dyn Trait）本身大小固定（16 字节：指针 + 长度/vtable）。
//!
//! 运行：`cargo run -p ch13_utility_traits --example 02_sized`

use ch13_utility_traits::section;
use std::mem::size_of;

fn main() {
    section("固定大小类型 vs DST");
    println!("  size_of::<i32>()    = {} 字节", size_of::<i32>());
    println!("  size_of::<[i32; 4]>() = {} 字节", size_of::<[i32; 4]>());
    // 切片 [i32] 是 DST，size_of 不能直接测；只能测「指向它的引用」。
    println!("  size_of::<&[i32]>() = {} 字节（胖指针：数据指针 + 长度）", size_of::<&[i32]>());
    println!("  size_of::<&str>()   = {} 字节（胖指针：数据指针 + 长度）", size_of::<&str>());
    println!("  size_of::<String>() = {} 字节（拥有：指针 + 长度 + 容量）", size_of::<String>());

    section("trait object 也是 DST —— &dyn Trait 是胖指针");
    trait Greet {
        fn hello(&self) -> &str;
    }
    impl Greet for u32 {
        fn hello(&self) -> &str {
            "u32"
        }
    }
    impl Greet for String {
        fn hello(&self) -> &str {
            "String"
        }
    }
    // &dyn Greet 是胖指针：数据指针 + vtable 指针。
    println!("  size_of::<&dyn Greet>() = {} 字节", size_of::<&dyn Greet>());
    let n: u32 = 5;
    let g: &dyn Greet = &n;
    println!("  dyn Greet.hello() = {}", g.hello());

    section("?Sized：泛型放宽，允许 DST 作为「引用/指针」的目标");
    // 注意：[T] 切片要求 T: Sized（元素必须是固定大小）。
    // ?Sized 的真正用武之地是「让函数能接受指向 DST 的引用」。
    // 例如标准库 Box<T: ?Sized>、Rc<T: ?Sized>、&T where T: ?Sized。
    fn print_len<T: ?Sized + std::fmt::Display>(value: &T) {
        // value 可能是 &str、&dyn Trait 这种「指向 DST 的胖指针」。
        println!("  print_len: {value}");
    }
    let s: &str = "hello"; // str 是 DST，&str 是胖指针
    print_len(s);
    print_len(&42_i32); // i32 是 Sized，也兼容

    section("Box<T: ?Sized>：可以把 DST 放进 Box");
    // Box<str> 和 Box<[u8]> 合法 —— Box 存胖指针。
    let boxed_str: Box<str> = "hi there".into();
    println!("  Box<str> = {boxed_str}（大小 {} 字节）", size_of::<Box<str>>());
    let boxed_slice: Box<[u8]> = vec![1, 2, 3].into_boxed_slice();
    println!("  Box<[u8]> = {:?}", boxed_slice);

    section("Sized 作为默认 bound 的实际影响");
    // fn f<T>() {} 隐含 T: Sized。
    // 想存「大小未知」的值到结构体？只能用指针（Box/&/Rc）。
    // struct HoldsStr { s: str }  // ❌ 编译失败：str 是 DST
    // struct HoldsStr<'a> { s: &'a str }  // ✅ 用引用
    println!("  （结构体字段必须是 Sized；要存 DST 只能用 Box/& 等指针）");
}
