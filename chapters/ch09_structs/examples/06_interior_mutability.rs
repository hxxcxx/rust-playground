//! 9.6 内部可变性：Cell + RefCell
//!
//! 关键结论：
//! - Rust 默认：不可变值内的字段也是不可变的（哪怕字段本身声明 `mut`）。
//! - 内部可变性打破这一规则：在外壳不可变的前提下，让内部数据可变。
//! - `Cell<T>`：仅适合 `T: Copy`，通过 `get()/set()` 工作；无运行时开销。
//! - `RefCell<T>`：适合任意 T；通过 `borrow()/borrow_mut()` 工作；
//!   违反借用规则时运行时 panic（不是编译期）。
//! - `Cell`/`RefCell` 都不是 `Sync` —— 不能跨线程共享（见第 19 章的 Mutex/Atomics）。
//!
//! 运行：`cargo run -p ch09_structs --example 06_interior_mutability`

use ch09_structs::{LogBuffer, SpiderRobot, section};
use std::cell::{Cell, RefCell};

fn main() {
    section("Cell<T>：Copy 类型的内部可变性");
    let cell: Cell<i32> = Cell::new(10);
    println!("  初始 get() = {}", cell.get());
    cell.set(20); // 注意：set 的 self 是 &self（不是 &mut self）！
    println!("  set(20) 后 get() = {}", cell.get());

    section("在「不可变」结构体中修改字段（SpiderRobot）");
    // robot 没有声明 mut，但我们能 add_hardware_error —— 这就是内部可变性
    let robot = SpiderRobot::new("Orb Weaver");
    println!("  初始错误数: {}", robot.hardware_error_count.get());
    robot.add_hardware_error();
    robot.add_hardware_error();
    robot.add_hardware_error();
    println!(
        "  3 次后错误数: {} (has_errors={})",
        robot.hardware_error_count.get(),
        robot.has_hardware_errors()
    );

    section("RefCell<T>：非 Copy 类型的内部可变性（运行时借用检查）");
    let buf = LogBuffer::new();
    buf.log("first");
    buf.log("second");
    buf.log("third");
    println!("  snapshot = {:?}", buf.snapshot());

    section("RefCell 的运行时借用规则：违反会 panic");
    let r = RefCell::new(String::from("hello"));
    {
        let _shared1 = r.borrow(); // 共享借用 OK
        let _shared2 = r.borrow(); // 多个共享借用 OK
        // let _mut = r.borrow_mut(); // ❌ panic：已被共享借用
    }
    {
        let _mut = r.borrow_mut(); // 可变借用独占 OK
        // let _shared = r.borrow(); // ❌ panic：已被可变借用
    }
    println!("  分别作用域：共享借用与可变借用各自 OK");

    section("try_borrow / try_borrow_mut：返回 Result 而非 panic");
    let r = RefCell::new(0);
    let b1 = r.borrow();
    let b2_result = r.try_borrow_mut();
    println!(
        "  已共享借用 → try_borrow_mut = {:?}",
        b2_result.map(|_| "ok").err()
    );
    drop(b1); // 显式释放
    let b3_result = r.try_borrow_mut();
    println!(
        "  释放后 → try_borrow_mut = {:?}",
        b3_result.map(|_| "ok").err().is_none()
    );

    section("Cell/RefCell 不是 Sync：不能跨线程");
    println!("  Cell / RefCell 适合单线程场景");
    println!("  跨线程要用 Mutex<T> / RwLock<T> / AtomicXxx（第 19 章）");
}
