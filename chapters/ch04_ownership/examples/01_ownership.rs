//! 4.1 所有权
//!
//! 关键结论：
//! - 每个值有唯一所有者；所有者离开作用域被 drop 时，它拥有的所有值一起被 drop。
//! - 变量拥有值；结构体拥有字段；元组/数组/Vec 拥有元素。
//! - 因此所有权关系构成一棵以某个变量为根的「所有权树」。
//!
//! 运行：`cargo run -p ch04_ownership --example 01_ownership`

use ch04_ownership::{Person, section};

/// 演示：变量拥有值，函数结束时整棵所有权树被丢弃。
///
/// `padovan` 的三个字（指针/容量/长度）位于栈帧；元素缓冲区位于堆。
/// 函数结束时，`padovan`（所有者）被 drop → 缓冲区（被拥有者）随之释放。
fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // 在此处分配
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {padovan:?}");
} // 在此处丢弃：padovan 离开作用域 → Vec drop → 堆缓冲区被释放

/// 演示：`Box<T>` 是另一个典型的所有者，它拥有其在堆上分配的 `T`。
/// `Box` 被 drop 时，它指向的堆空间也会被释放。
fn box_example() {
    {
        // Box::new 把元组 (0.625, 0.5) 移动到堆上，并返回指向它的 Box
        let point = Box::new((0.625, 0.5)); // 在此处分配 point
        let label = format!("{point:?}"); // 在此处分配 label
        assert_eq!(label, "(0.625, 0.5)");
        println!("point = {point:?}, label = {label:?}");
    } // 在此处丢弃两者：point 的堆空间、label 的堆缓冲区都被释放
}

/// 演示：更复杂的所有权树 —— `Vec<Person>` 拥有元素，每个 `Person` 拥有 `String`，
/// 每个 `String` 拥有堆上的文本缓冲区。所有权链一路向下构成一棵树。
fn composers_example() {
    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });

    // 用引用遍历，不移动所有权（第5章详述引用）
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
    // composers 离开作用域：Vec → 每个 Person → 每个 String → 文本缓冲区，全部被释放
}

fn main() {
    section("所有权规则①：变量拥有值，离开作用域即被丢弃");
    print_padovan();

    section("所有权规则②：Box<T> 拥有其在堆上分配的 T");
    box_example();

    section("所有权规则③：结构体/Vec 拥有字段/元素，形成所有权树");
    composers_example();

    section("总结");
    println!("每个值都有唯一所有者，所有者被丢弃时其拥有的值一并被丢弃。");
    println!("这让我们仅凭阅读代码就能确定任意值的生命周期。");
}
