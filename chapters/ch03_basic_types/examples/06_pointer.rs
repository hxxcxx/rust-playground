//! 3.6 指针类型：引用、Box、裸指针
//!
//! 运行：`cargo run -p ch03_basic_types --example 06_pointer`
//
// 教学性示例：演示裸指针必须放在 unsafe 块中使用。
#![allow(unsafe_code)]

use ch03_basic_types::section;

fn main() {
    section("共享引用 &T：只读、可多个");
    let s = String::from("hello");
    let r1: &String = &s;
    let r2: &String = &s; // 同时存在多个共享引用
    println!("r1 = {r1}, r2 = {r2}");

    section("可变引用 &mut T：独占、可改");
    let mut v = vec![1, 2, 3];
    let r: &mut Vec<i32> = &mut v;
    r.push(4); // 通过可变引用修改
    println!("v = {v:?}");
    // 只要 r 还活着，就不能再借用 v（独占）

    section("引用是胖指针（对切片/str 而言）");
    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    // &[i32] 是 (ptr, len) 两个机器字
    println!("slice ptr+len = {slice:?}, len = {}", slice.len());
    println!(
        "size_of::<&[i32; 5]>() = {} bytes（普通引用）",
        size_of::<&[i32; 5]>()
    );
    println!(
        "size_of::<&[i32]>()   = {} bytes（胖指针）",
        size_of::<&[i32]>()
    );

    section("Box<T>：堆分配，独占所有权");
    let t = (12, "eggs");
    let b: Box<(i32, &str)> = Box::new(t);
    println!(
        "Box = {b:?}, size_of::<Box<_>>() = {} bytes",
        size_of_val(&b)
    );
    // b 离开作用域时，堆内存立即释放（除非被 move）

    section("递归类型必须用 Box（编译期大小已知）");
    // 链表节点：直接嵌套会无限大，必须 Box<Recursive>
    let list: List = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("list 累加 = {}", list.sum());
    println!("size_of::<List>() = {} bytes", size_of::<List>());

    section("裸指针 *const T / *mut T：只能在 unsafe 块解引用");
    let x = 42_u32;
    let raw: *const u32 = &x;
    unsafe {
        assert_eq!(*raw, 42);
        println!("*raw = {}", *raw);
    }
    // safe Rust 中无法解引用裸指针 —— 安全保证仍成立
}

/// 递归链表：典型的 Box 应用场景。
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn sum(&self) -> i32 {
        match self {
            List::Cons(v, rest) => v + rest.sum(),
            List::Nil => 0,
        }
    }
}
