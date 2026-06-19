//! 14.4 把闭包存进结构体（回调 / 事件处理）
//!
//! 关键结论：
//! - 闭包可以作为「字段」存进结构体（泛型参数 F: Fn）。
//! - 每个「闭包字面量」有独特的匿名类型 → 结构体必须用泛型 F 承载。
//! - 实战：GUI 按钮 onClick、定时器回调、事件分发器、配置回调。
//! - 想「同一个结构体存不同类型闭包」→ 用 Box<dyn Fn()>（动态分发）。
//!
//! 运行：`cargo run -p ch14_closures --example 04_storing`

use ch14_closures::{Button, section};

fn main() {
    section("把闭包存进 Button（泛型 F: Fn）");
    let click_count = std::cell::Cell::new(0);
    // 闭包捕获了 click_count 的引用 —— 存进 Button。
    let btn = Button::new("Submit", || {
        click_count.set(click_count.get() + 1);
        println!("    → 提交被点击！（第 {} 次）", click_count.get());
    });
    btn.click();
    btn.click();
    btn.click();
    println!("  Button 标签: {}", btn.label);

    section("不同闭包 → 不同类型的 Button（泛型）");
    // Button<F> 的 F 不同 → btn1 和 btn2 类型不同（每个闭包是独立匿名类型）。
    // 不能写 `Button<impl Fn()>`（impl Trait 不能用在变量类型位置），让编译器推断。
    let btn1 = Button::new("OK", || println!("    → OK 被点击"));
    let btn2 = Button::new("Cancel", || println!("    → 取消"));
    btn1.click();
    btn2.click();

    section("捕获变量的闭包存进结构体");
    let log_prefix = String::from("[APP]");
    // 闭包 move 捕获 log_prefix，存进结构体。
    let btn3 = Button::new("Log", move || {
        println!("    {log_prefix} 日志按钮被点击");
    });
    btn3.click();
    // log_prefix 已 move，不能再用。

    section("事件分发器：用 Box<dyn Fn> 存多种回调");
    let mut dispatcher = EventDispatcher::new();
    dispatcher.on("click", Box::new(|| println!("    处理 click")));
    dispatcher.on("click", Box::new(|| println!("    记录 click 日志")));
    dispatcher.on("hover", Box::new(|| println!("    处理 hover")));
    dispatcher.trigger("click");
    dispatcher.trigger("hover");

    section("闭包与生命周期：捕获的引用必须比结构体长寿");
    let external = String::from("外部数据");
    // 这个闭包借用 external —— Button 必须在 external 还活着时使用。
    let btn4 = Button::new("Read", || println!("    读取: {external}"));
    btn4.click();
    // external 在 btn4 之后销毁（顺序保证安全）。
}

/// 事件分发器：用 Box<dyn Fn()> 存储「不同类型」的回调。
struct EventDispatcher {
    handlers: std::collections::HashMap<String, Vec<Box<dyn Fn()>>>,
}

impl EventDispatcher {
    fn new() -> Self {
        Self {
            handlers: std::collections::HashMap::new(),
        }
    }

    fn on(&mut self, event: &str, handler: Box<dyn Fn()>) {
        self.handlers
            .entry(event.to_string())
            .or_default()
            .push(handler);
    }

    fn trigger(&self, event: &str) {
        if let Some(handlers) = self.handlers.get(event) {
            println!("  触发事件 '{event}'：");
            for h in handlers {
                h();
            }
        }
    }
}
