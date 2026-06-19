//! 14.5 实战：闭包用于排序（sort_by / 自定义排序键）
//!
//! 关键结论：
//! - `sort_by(|a, b| ...)` / `sort_by_key(|x| ...)`：用闭包定义排序规则。
//! - 闭包捕获「外部上下文」（如阈值、方向），让排序逻辑更灵活。
//! - `sort_by_key` 要求返回 Ord 的「键」；`sort_by` 接受任意比较函数。
//! - 排序稳定 / 不稳定：`sort`（不稳定、快）/ `sort_stable`（稳定、慢一点）。
//!
//! 运行：`cargo run -p ch14_closures --example 05_sorting`

use ch14_closures::{City, section};

fn main() {
    section("sort_by_key：按人口升序");
    let mut cities = sample_cities();
    cities.sort_by_key(|c| c.population);
    for c in &cities {
        println!("  {}: {}", c.name, c.population);
    }

    section("sort_by_key + 闭包捕获：按人口降序");
    let mut cities = sample_cities();
    // 闭包返回「负人口」实现降序（或用 reverse）。
    cities.sort_by_key(|c| std::cmp::Reverse(c.population));
    for c in &cities {
        println!("  {}: {}", c.name, c.population);
    }

    section("sort_by：自定义比较函数（多字段）");
    let mut cities = sample_cities();
    // 先按名字长度，再按人口。
    cities.sort_by(|a, b| {
        a.name.len().cmp(&b.name.len()).then(b.population.cmp(&a.population))
    });
    for c in &cities {
        println!("  {} (名字长{}, 人口{})", c.name, c.name.len(), c.population);
    }

    section("闭包捕获外部变量：动态排序方向");
    let mut cities = sample_cities();
    sort_population(&mut cities, true); // 升序
    println!("  升序:");
    for c in &cities {
        println!("    {}", c.name);
    }
    sort_population(&mut cities, false); // 降序
    println!("  降序:");
    for c in &cities {
        println!("    {}", c.name);
    }

    section("max_by / min_by：用闭包找极值");
    let cities = sample_cities();
    let biggest = cities.iter().max_by_key(|c| c.population).unwrap();
    let smallest = cities.iter().min_by_key(|c| c.population).unwrap();
    println!("  最大城市: {} ({})", biggest.name, biggest.population);
    println!("  最小城市: {} ({})", smallest.name, smallest.population);

    section("闭包 + 迭代器链：取人口前 3 的城市名");
    let mut cities = sample_cities();
    cities.sort_by_key(|c| std::cmp::Reverse(c.population));
    let top3: Vec<&str> = cities.iter().take(3).map(|c| c.name.as_str()).collect();
    println!("  Top 3: {top3:?}");
}

/// 取样城市列表。
fn sample_cities() -> Vec<City> {
    vec![
        City::new("Shanghai", 24_870_000),
        City::new("Beijing", 21_890_000),
        City::new("Guangzhou", 18_670_000),
        City::new("Shenzhen", 17_560_000),
        City::new("Hangzhou", 12_370_000),
    ]
}

/// 按人口排序，方向由 ascending 控制。
fn sort_population(cities: &mut [City], ascending: bool) {
    cities.sort_by(|a, b| {
        let ord = a.population.cmp(&b.population);
        if ascending {
            ord
        } else {
            ord.reverse()
        }
    });
}
