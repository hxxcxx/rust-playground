//! 5.4 共享与可变
//!
//! 关键结论：
//! - 两条核心规则：
//!   ① 「共享访问是只读访问」：共享引用借用的值在整个借用期内被冻结 ——
//!   不能赋值、不能移动、不能再被可变借用，连所有者也只能只读。
//!   ② 「可变访问是独占访问」：可变引用借用的值，在整个借用期内只能通过这一条路径访问 ——
//!   其它任何引用、所有者的直接访问都被禁止。
//! - 这两条规则同时覆盖「从被引用对象可达的所有值」—— 整条所有权路径都不能改动。
//! - 因此 Rust 在编译期就能拦截：迭代器失效、自赋值 bug、并发数据竞争。
//! - 从可变引用可以再借用：`&mut *m`（独占再独占/独占再共享，期间不重叠）。
//! - Rust 的共享引用比 C 的 `const T*` 严格得多 —— 后者无法保证常量性。
//!
//! 运行：`cargo run -p ch05_references --example 06_shared_vs_mutable`

use ch05_references::section;

/// ❌ 反面教材（注释保留）：共享引用存在期间，被引用对象不能被「移动」。
/// 报错：error[E0505]: cannot move out of `v` because it is borrowed
fn move_while_borrowed_commented() {
    println!("❌ 反面教材（见源码注释）：");
    println!("let v = vec![4, 8, 19, 27, 34, 10];");
    println!("let r = &v;        // 共享借用 v");
    println!("let aside = v;     // ❌ 试图在借用期间 move v —— 编译失败");
    println!("r[0];              // r 会变成悬空指针");
    // let v = vec![4, 8, 19, 27, 34, 10];
    // let r = &v;
    // let aside = v;
    // r[0];
    // let _ = (aside,);
}

/// ✅ 正例：让引用的生命周期在移动之前结束。
fn borrow_ends_before_move() {
    let v = vec![4, 8, 19, 27, 34, 10];
    {
        let r = &v;
        assert_eq!(r[0], 4); // 借用期间 v 仍可读
        println!("✅ 借用期间：r[0] = {}", r[0]);
    } // r 在此结束，借用释放
    let aside = v; // 现在可以移动
    println!("✅ 借用结束后 move：aside = {aside:?}");
}

/// ❌ 反面教材（注释保留）：向 Vec 追加它自己的元素引用。
/// `extend(&mut wave, &wave)` 会同时持有 wave 的可变引用和共享引用 —— 违反独占规则。
/// 即便不真正出 UB，重分配可能让 slice 变悬空指针，编译期就要拒绝。
/// 报错：error[E0502]: cannot borrow `wave` as immutable because it is also borrowed as mutable
fn extend_self_commented() {
    println!("\n❌ 反面教材（见源码注释）：");
    println!("let mut wave = vec![0.0, 1.0, 0.0, -1.0];");
    println!("extend(&mut wave, &wave); // ❌ 同时 &mut wave 和 &wave —— 编译失败");
    println!("（重分配可能让共享引用指向已释放的旧缓冲区 → 编译期拒绝）");
}

/// 演示：标准库的 `Vec::extend_from_slice` 同样遵守独占规则。
fn extend_from_other_vec() {
    let mut wave = Vec::new();
    let head = vec![0.0_f64, 1.0];
    let tail = [0.0_f64, -1.0];
    wave.extend_from_slice(&head); // 从另一个 Vec 借用
    wave.extend_from_slice(&tail); // 从数组借用
    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);
    println!("✅ 借用不同的来源构建 wave = {wave:?}");
}

/// 演示：共享引用期间禁止任何修改 —— 即使通过所有者也不行。
/// 这是 Rust 与 C `const T*` 最大的差异。
fn shared_freezes_owner() {
    let mut x = 42_i32;
    let p = &x; // 共享引用：x 在此被冻结为只读
    assert_eq!(*p, 42);
    // x += 1;  // ❌ cannot assign to `x` because it is borrowed
    println!("✅ 共享引用存在时 x 被冻结：*p = {p}，不能修改 x");
    // 在 p 不再使用之后才能修改 x：
    let _used = *p;
    x += 1;
    println!("✅ 共享引用生命周期结束后：x = {x}");
}

/// 演示：可变引用独占期间，任何其它路径（包括所有者）都被禁止。
fn mutable_is_exclusive() {
    let mut y = 20_i32;
    let m1 = &mut y; // 独占借用
    *m1 += 1;
    // 在 m1 借用期间，下列访问都违法：
    // let m2 = &mut y; // ❌ cannot borrow `y` as mutable more than once
    // let z = y;        // ❌ cannot use `y` because it was mutably borrowed
    // println!("{y}");  // ❌ 不可变借用 y 也不行
    // 通过 m1 这条唯一路径读取，是允许的：
    let modified = *m1;
    println!("✅ 可变引用独占：通过 *m1 修改后 y = {modified}（仅能通过 m1 访问）");
}

/// 演示：从可变引用「再借用」是允许的，只要借用期不重叠。
fn reborrow_from_mutable() {
    let mut v = (136, 139);
    let m = &mut v; // 独占借用 v
    {
        let m0 = &mut m.0; // 从 m 再借用 m.0 —— 允许，与 m 错开
        *m0 = 137;
        println!("✅ 再借用 &mut m.0：v.0 修改为 {m0}");
    } // m0 结束
    {
        let r1 = &m.1; // 从 m 再借用 m.1 的共享引用 —— 允许
        assert_eq!(*r1, 139);
        println!("✅ 再借用 &m.1（共享）：*r1 = {r1}");
    }
    // v.1; // ❌ 通过其它路径访问仍被禁止（在 m 借用期内）
    let _ = (m,);
}

/// 演示：自赋值陷阱 —— C/C++ 中 `f = f` 会让 close 在 dup 前关闭描述符，
/// Rust 因独占规则在编译期就拒绝 `clone_from(&mut f, &f)`。
fn self_assignment_trap_commented() {
    println!("\n❌ 反面教材（见源码注释）：C 的「自赋值」陷阱在 Rust 中被编译期拦截");
    println!("struct File {{ descriptor: i32 }}");
    println!("fn clone_from(this: &mut File, rhs: &File) {{ ... }}");
    println!("clone_from(&mut f, &f); // ❌ 同时 &mut f 和 &f —— 编译失败");
    println!("→ 等价于「迭代器失效」/「memcpy 源目重叠」这类经典 bug，编译期根除。");
    // struct File { descriptor: i32 }
    // fn clone_from(this: &mut File, rhs: &File) {
    //     // this.descriptor = dup(rhs.descriptor); // 假装 close/dup
    //     let _ = (this, rhs);
    // }
    // let mut f = File { descriptor: 3 };
    // clone_from(&mut f, &f);
}

fn main() {
    section("借用期间不能移动被引用对象");
    move_while_borrowed_commented();

    section("借用结束之后再移动 —— 合法");
    borrow_ends_before_move();

    section("向 Vec 追加自身引用为何被拒绝");
    extend_self_commented();

    section("合法：用 extend_from_slice 借用别的来源");
    extend_from_other_vec();

    section("共享引用会冻结所有者（与 C const 不同）");
    shared_freezes_owner();

    section("可变引用是独占的");
    mutable_is_exclusive();

    section("从可变引用再借用（非重叠）");
    reborrow_from_mutable();

    section("自赋值陷阱被编译期拦截");
    self_assignment_trap_commented();

    section("总结");
    println!("① 共享 = 只读：借用期内整条所有权路径冻结");
    println!("② 可变 = 独占：借用期内整条所有权路径不可访问");
    println!("→ 消除迭代器失效、自赋值、数据竞争等大批 bug，零运行时开销。");
}
