//! 3.4 bool 与 char：布尔语义、Unicode 字符
//!
//! 运行：`cargo run -p ch03_basic_types --example 04_bool_char`

use ch03_basic_types::section;

fn main() {
    section("bool：只有 true / false，控制流严格要求 bool");
    let x = 5;
    // 不能写 `if x { ... }` —— Rust 不像 C/C++ 隐式转换整数到 bool
    let is_nonzero = x != 0;
    assert!(is_nonzero);
    println!("x != 0 = {is_nonzero}");

    // bool 占 1 个字节（便于取地址）
    println!("size_of::<bool>() = {} byte", std::mem::size_of::<bool>());

    section("bool 与整数：仅支持 bool → 整数，反之需要显式比较");
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
    // 反向：`1 as bool` 是编译错误；必须写 `1 != 0`
    let from_int = 1_i32 != 0;
    println!("true as i32 = {}, from_int = {}", true as i32, from_int);

    section("char：32 位 Unicode 标量值（不是 UTF-8 字节，不是 u8）");
    let ascii = '*';
    let chinese = '字';
    let kanji = '錆'; // 日语"锈"
    let emoji_ok = '√';
    println!("'{ascii}' '{chinese}' '{kanji}' '{emoji_ok}'");
    println!("size_of::<char>() = {} bytes", std::mem::size_of::<char>());

    section("转义序列");
    assert_eq!('\'', '\''); // 单引号
    assert_eq!('\\', '\\'); // 反斜杠
    assert_eq!('\n', '\x0a'); // 换行
    assert_eq!('\u{CA0}', 'ಠ'); // 卡纳达语字符
    println!("\\u{{CA0}} = 'ಠ'");

    section("char ↔ 整数：as 可截断高位");
    assert_eq!('*' as i32, 42); // '*' 的 ASCII 码是 42
    assert_eq!('ಠ' as u16, 0xca0);
    assert_eq!('ಠ' as i8, -0x60); // 0xca0 截断为 8 位 → 0x60 → -96
    println!("'ಠ' as u16 = 0x{:x}, as i8 = {}", 'ಠ' as u16, 'ಠ' as i8);

    section("反向转换：仅 u8 可直接 as，其他用 std::char::from_u32");
    assert_eq!(42_u8 as char, '*');
    let from_u32 = std::char::from_u32(0xCA0);
    assert_eq!(from_u32, Some('ಠ'));
    let invalid = std::char::from_u32(0xD800); // 代理对码点，非法
    assert_eq!(invalid, None);
    println!(
        "from_u32(0xCA0) = {:?}, from_u32(0xD800) = {:?}",
        from_u32, invalid
    );

    section("char 的常用方法");
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8)); // 十进制
    assert_eq!('f'.to_digit(16), Some(15)); // 十六进制
    assert_eq!('ಠ'.len_utf8(), 3); // 该字符在 UTF-8 中占 3 字节
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
    println!("'ಠ' 在 UTF-8 中占 {} 字节", 'ಠ'.len_utf8());
}
