//! 第14章 闭包 —— 入口。
//!
//! 章节示例：
//! - `01_capturing`    —— 捕获变量：& / &mut / move 三种方式
//! - `02_fn_traits`    —— Fn / FnMut / FnOnce：作为函数参数
//! - `03_returning`    —— 返回闭包：impl Fn / Box<dyn Fn>
//! - `04_storing`      —— 把闭包存进结构体（回调/事件处理）
//! - `05_sorting`      —— 实战：sort_by / 自定义排序键
//! - `06_performance`  —— 闭包性能：零开销、与函数指针对比

fn main() {
    println!("第14章 闭包");
    println!();
    println!("运行示例：");
    println!("  cargo run -p ch14_closures --example 01_capturing");
    println!("  cargo run -p ch14_closures --example 02_fn_traits");
    println!("  cargo run -p ch14_closures --example 03_returning");
    println!("  cargo run -p ch14_closures --example 04_storing");
    println!("  cargo run -p ch14_closures --example 05_sorting");
    println!("  cargo run -p ch14_closures --example 06_performance");
}
