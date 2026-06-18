//! 6.5 `break` / `continue` 与循环标签
//!
//! 关键结论：
//! - `break`：仅用于循环（`while`/`for`/`loop`），`match` 不需要它。
//! - `continue`：跳到下一次迭代。
//! - 嵌套循环可用 `'label:` 标记，`break 'label` / `continue 'label` 跨层。
//! - `break 'label value`：可同时带标签和返回值。
//!
//! 运行：`cargo run -p ch06_expressions --example 05_break_continue`

use ch06_expressions::section;

fn main() {
    section("break：提前退出当前循环");
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
        if sum > 10 {
            println!("  在 i={i} 时 sum={sum} > 10，提前退出");
            break;
        }
    }

    section("continue：跳过本次迭代");
    for i in 0..6 {
        if i % 2 == 0 {
            continue; // 跳过偶数
        }
        print!("  {i}");
    }
    println!();

    section("标签 'outer: 跳出外层循环");
    'outer: for row in 0..3 {
        for col in 0..3 {
            if row == 1 && col == 1 {
                println!("  找到 (1,1)，break 'outer 跳出全部");
                break 'outer; // 一次跳出两层
            }
            print!("  ({row},{col})");
        }
        println!();
    }

    section("break 'label value：带标签和返回值");
    // 找到二维矩阵中第一个负数的位置（用 loop 配合迭代器与标签）
    let matrix: [[i32; 3]; 3] = [[1, 2, 3], [4, -5, 6], [7, 8, 9]];
    let negative: Option<(usize, usize)> = 'find: {
        for (r, row) in matrix.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                if val < 0 {
                    break 'find Some((r, c));
                }
            }
        }
        None
    };
    println!("  第一个负数位于: {negative:?}");

    section("continue 'label：直接进入外层循环的下一轮");
    'rows: for r in 0..3 {
        for c in 0..3 {
            if c == 1 {
                println!("  跳过 row={r} 剩下的列");
                continue 'rows;
            }
            println!("  处理 ({r},{c})");
        }
    }
}
