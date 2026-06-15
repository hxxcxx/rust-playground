fn main() {
    println!("Hello, Rust!");

    let greeting = "Hello";
    let target = "Rust";
    println!("{greeting}, {target}!");

    let sum: i32 = (1..=10).sum();
    println!("1..=10 的和 = {sum}");
}
