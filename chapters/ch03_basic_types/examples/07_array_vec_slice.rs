//! 3.7 数组 `[T; N]`、向量 `Vec<T>`、切片 `&[T]` / `&mut [T]`
//!
//! 运行：`cargo run -p ch03_basic_types --example 07_array_vec_slice`

use ch03_basic_types::section;

fn main() {
    section("数组 [T; N]：长度是类型的一部分，编译期固定");
    let lazy: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxa = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(lazy[3], 7);
    assert_eq!(taxa.len(), 3);
    println!("lazy = {lazy:?}, taxa = {taxa:?}");

    section("[V; N] 语法：用 V 填充 N 个元素");
    let mut sieve = [true; 10000]; // 1 万个 true
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false; // 标记合数
                j += i;
            }
        }
    }
    assert!(sieve[211]); // 211 是素数
    assert!(!sieve[9876]); // 9876 是合数
    println!(
        "埃氏筛：211 是素数 ? {}, 9876 是素数 ? {}",
        sieve[211], sieve[9876]
    );

    section("Vec<T>：堆分配、可动态增长");
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);
    println!("primes = {primes:?}");

    section("Vec 的构造方式");
    let v1: Vec<u8> = vec![0; 10]; // 重复
    let v2: Vec<i32> = (1..=5).collect(); // 从迭代器
    let v3: Vec<&str> = ["step", "on", "no", "pets"].to_vec();
    println!("v1(前5) = {:?}, v2 = {v2:?}, v3 = {v3:?}", &v1[..5]);

    section("Vec::with_capacity：避免多次重分配");
    let mut buf: Vec<i32> = Vec::with_capacity(3);
    println!("初始: len={}, cap={}", buf.len(), buf.capacity());
    buf.push(1);
    buf.push(2);
    buf.push(3); // 不需要重新分配
    assert_eq!(buf.capacity(), 3);
    buf.push(4); // 触发扩容（通常翻倍）
    println!("push 4 个元素后: len={}, cap={}", buf.len(), buf.capacity());

    section("Vec 的增删：insert / remove / pop");
    let mut v = vec![10, 20, 30, 40, 50];
    v.insert(3, 35); // [10,20,30,35,40,50]
    v.remove(1); // [10,30,35,40,50]
    assert_eq!(v, [10, 30, 35, 40, 50]);
    assert_eq!(v.pop(), Some(50));
    assert_eq!(v.pop(), Some(40));
    assert_eq!(v.pop(), Some(35));
    assert_eq!(v.pop(), Some(30));
    assert_eq!(v.pop(), Some(10));
    assert_eq!(v.pop(), None);
    println!("pop 后已清空");

    section("切片 &[T] / &mut [T]：胖指针，对数组/Vec 的部分借用");
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort(); // 隐式从 &mut [i32; 5] 借用 &mut [i32]
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
    println!("排序后 = {chaos:?}");

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let sv: &[f64] = &v; // &Vec<f64> → &[f64]
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];
    let sa: &[f64] = &a; // &[f64; 4] → &[f64]
    print_slice(sv);
    print_slice(sa);
    print_slice(&v[0..2]); // 范围切片
    print_slice(&a[2..]);
    print_slice(&sv[1..3]);

    section("size_of 对比：数组的 size 与 Vec 不同");
    println!(
        "size_of [i32; 4]  = {} bytes（直接内嵌）",
        size_of::<[i32; 4]>()
    );
    println!(
        "size_of Vec<i32>  = {} bytes（指针+cap+len）",
        size_of::<Vec<i32>>()
    );
    println!(
        "size_of &[i32]    = {} bytes（胖指针）",
        size_of::<&[i32]>()
    );
    println!(
        "size_of &[i32; 4] = {} bytes（普通引用）",
        size_of::<&[i32; 4]>()
    );
}

/// 数组和向量都能通过切片引用传入。
fn print_slice(n: &[f64]) {
    print!("[");
    for (i, v) in n.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{v}");
    }
    println!("]");
}
