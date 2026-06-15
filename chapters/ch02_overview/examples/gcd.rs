//! 示例 1：命令行 GCD 计算器
//!
//! 运行：`cargo run -p ch02_overview --example gcd 42 56 14`

use std::env;
use std::process;
use std::str::FromStr;

use ch02_overview::gcd;

fn main() {
    let numbers = vec![42, 56, 15];

    // reduce：依次对相邻元素求 gcd，得到全体数的 gcd
    let d = numbers.iter().copied().reduce(gcd).unwrap();
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
