//! 3.8 字符串类型：&str / String / 字节字符串 / 原始字符串
//!
//! 运行：`cargo run -p ch03_basic_types --example 08_strings`

use ch03_basic_types::section;

fn main() {
    section("字符串字面量：用 \\\\ 转义或续行");
    let speech = "\"Ouch!\" said the well.\n";
    println!("{speech}");

    let multi = "In the room the women come and go,
Singing of Mount Abora";
    println!("{multi}");

    let continued = "It was a bright, cold day in April, and \
there were four of us—more or less."; // 行尾 \\ 续行：忽略换行和下一行缩进
    println!("{continued}");

    section("原始字符串 r\"...\"：不处理任何转义");
    let win_path = r"C:\Program Files\Gorillas";
    let regex_pat = r"\d+(\.\d+)*";
    println!("{win_path}\n{regex_pat}");

    section("原始字符串 + 井号：可包含双引号");
    println!(
        "{}",
        r###"
This raw string started with 'r###"'.
It can contain '"' freely.
Ends only at '" followed by three #'s: "###
    );

    section("字节字符串 b\"...\"：类型是 &[u8; N]");
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
    println!("method = {method:?}（注意：不是 &str，是 &[u8; 3]）");

    section("String vs &str：内存模型");
    // String：堆上的、可增长的 UTF-8 缓冲区，类似 Vec<u8>
    // &str：胖指针，指向某段 UTF-8 文本（可以是 String 内的切片，或字面量）
    let noodles = "noodles".to_string();
    let oodles: &str = &noodles[1..]; // 借用 noodles 的一部分
    let poodles: &str = "O_O"; // 字面量，位于静态区
    println!("noodles(String) = {noodles}");
    println!("oodles(&str) = {oodles}");
    println!("poodles(&str literal) = {poodles}");

    section("len() 返回字节数，不是字符数");
    assert_eq!("O_O".len(), 7); // '√' 是 3 字节 UTF-8
    assert_eq!("O_O".chars().count(), 3);
    println!(
        "\"O_O\".len() = {} 字节, .chars().count() = {} 字符",
        "O_O".len(),
        "O_O".chars().count()
    );

    section("创建 String 的几种方式");
    let s1 = "too many pets".to_string();
    let s2: String = format!("{}°{:02}′{:02}″N", 24, 5, 23);
    let bits = vec!["veni", "vidi", "vici"];
    let s3 = bits.concat();
    let s4 = bits.join(", ");
    println!("s1 = {s1}");
    println!("s2 = {s2}");
    println!("s3 = {s3}");
    println!("s4 = {s4}");

    section("str 的常用方法");
    assert!("peanut".contains("nut"));
    assert_eq!("O_O".replace("O", "■"), "■_■");
    assert_eq!("    clean\n ".trim(), "clean");
    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with('v'));
    }
    println!("{}", "ONE".to_lowercase() == "one");
    println!("\"  clean\\n \".trim() = {:?}", "    clean\n ".trim());

    section("Vec<u8> 与 String 的对应关系");
    println!(
        "size_of String = {} bytes（与 Vec<u8> 同构）",
        size_of::<String>()
    );
    println!("size_of &str   = {} bytes（胖指针）", size_of::<&str>());

    section("函数参数：优先 &str（可同时接受 &String 和字面量）");
    greet("world"); // 字面量
    let name = String::from("Alice");
    greet(&name); // &String 自动解引用为 &str
}

/// &str 参数最灵活：调用者可传字面量，也可传 &String。
fn greet(who: &str) {
    println!("Hello, {who}!");
}
