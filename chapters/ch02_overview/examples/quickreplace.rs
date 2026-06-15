//! 示例 4：带彩色输出的正则查找替换工具
//!
//! 运行：`cargo run --example quickreplace -- "world" "Rust" input.txt output.txt`

use std::{env, fs, process};

use owo_colors::OwoColorize;
use regex::Regex;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}",
            "Error:".red().bold(),
            args.len()
        );
        process::exit(1);
    }
    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).into_owned())
}

/// 打印错误并退出。返回 `!` 表示永不返回。
fn fail(context: impl AsRef<str>, detail: impl std::fmt::Debug) -> ! {
    eprintln!(
        "{} {}: {:?}",
        "Error:".red().bold(),
        context.as_ref(),
        detail
    );
    process::exit(1);
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => fail(format!("failed to read from '{}'", args.filename), e),
    };

    let replaced = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => fail("failed to replace text", e),
    };

    if let Err(e) = fs::write(&args.output, &replaced) {
        fail(format!("failed to write to '{}'", args.output), e);
    }
}
