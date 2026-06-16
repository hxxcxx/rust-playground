//! 4.2.3 / 4.2.4 移动与控制流、移动与索引内容
//!
//! 关键结论：
//! - 一般原则：若变量值「有可能」已被移动走，且之后未重新赋值，就视为未初始化。
//! - 在循环中移动变量值是禁止的（除非下次迭代前明确重新赋值）。
//! - 不能从 `Vec` 等集合的索引位置直接 move 出元素（否则 Vec 需要额外记录哪些元素已失效）。
//!   解决方法：`pop` / `swap_remove` / `std::mem::replace` / `Option::take` / `for ... in v`。
//!
//! 运行：`cargo run -p ch04_ownership --example 03_moves_in_collections`

use ch04_ownership::section;

/// 演示：在 if/else 分支中移动是允许的，但分支之后再使用就会报错。
#[allow(dead_code)]
fn move_in_branches() {
    let c = true;
    let x = vec![10, 20, 30];
    if c {
        // f(x); // 在 if 分支中从 x 移动是允许的
        println!("if 分支：x 仍然存活，我们只是借用它 {x:?}");
    } else {
        // g(x);
        println!("else 分支：同样允许在此移动 x");
    }
    // 注意：如果上面任一分支真的 move 了 x，下面这行就会编译失败：
    // h(x); // ❌ use of moved value `x`
    println!("分支结束后 x 仍然可用：{x:?}");
}

/// 演示：循环中 move 后必须在下次迭代前重新赋值。
#[allow(dead_code)]
fn move_in_loop_with_reassign() {
    let mut x = vec![10, 20, 30];
    let mut i = 0;
    while i < 2 {
        println!("迭代 {i}：消费 x = {x:?}");
        // g(x); // 此处 move 走 x
        let _moved = std::mem::take(&mut x); // 用 take 模拟「move 走」
        x = vec![100 + i, 200 + i]; // 必须给 x 重新赋值，否则下一次迭代会报错
        i += 1;
    }
    println!("循环结束后 x = {x:?}");
}

/// 演示：不能直接从 Vec 索引 move 出元素；要用三种「安全移出」方法。
fn move_out_of_vec() {
    // 构造 ["101", "102", "103", "104", "105"]
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // ❌ 不能这样：
    // let third = v[2]; // error[E0507]: cannot move out of index of `Vec<String>`

    // ① pop：从末尾移出
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    // ② swap_remove：把目标位置与最后一个元素交换后弹出（顺序会变）
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // ③ mem::replace：用一个替身占位，取出原值
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    println!("剩余 v = {v:?}");
    assert_eq!(v, vec!["101", "104", "substitute"]);
}

/// 演示：`for ... in v` 直接消费整个 Vec，每次迭代 move 一个元素到循环变量。
fn for_consumes_vec() {
    let v = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];

    // 这里把 v move 给 for 循环，v 此后不可再用。
    // 循环内部把每个 String move 到 s，于是可以在循环体里修改它。
    for mut s in v {
        s.push('!');
        println!("{s}");
    }
    // println!("{:?}", v); // ❌ v 已被 move
}

/// 演示：当编译器无法静态跟踪「是否持有值」时，用 Option 表达「可能为空」。
/// Option::take 是「从可变位置取出值，原位置变为 None」的标准做法。
#[allow(dead_code, clippy::useless_vec)]
fn option_take_pattern() {
    struct Person {
        name: Option<String>,
        birth: i32,
    }
    let mut composers = vec![Person {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    }];

    // ❌ 不能直接 move 字段：let first = composers[0].name;
    // 用 Option::take 取出 Some，原位置变成 None：
    let first_name = composers[0].name.take();
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);
    println!("take 后：first_name = {first_name:?}, composers[0].name = {:?}", composers[0].name);
}

fn main() {
    section("if/else 分支中的移动");
    move_in_branches();

    section("循环中移动 + 重新赋值");
    move_in_loop_with_reassign();

    section("从 Vec 移出元素的三种方法：pop / swap_remove / mem::replace");
    move_out_of_vec();

    section("for ... in v 直接消费 Vec");
    for_consumes_vec();

    section("Option::take：从「可能为空」的字段中移出值");
    option_take_pattern();
}
