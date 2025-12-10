// intro1.rs
// 移除掉题目中提到的 `I AM NOT DONE` 注释（本题原本就没有，直接保留代码即可）

fn main() {
    // 基础 println! 宏：输出普通字符串
    println!("Hello and");
    // r#"..."# 是原始字符串语法：无需转义特殊字符（这里主要是空格/下划线）
    println!(r#"       welcome to...                      "#);
    println!(r#"                 _   _ _                  "#);
    println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
    println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
    println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
    println!(r#" |_|   \__,_|___/\__|_|_|_| |_|\__, |___/ "#);
    println!(r#"                               |___/      "#);
    // 空 println!：输出一个换行符
    println!();
    // 多行普通字符串输出
    println!("This exercise compiles successfully. The remaining exercises contain a compiler");
    println!("or logic error. The central concept behind Rustlings is to fix these errors and");
    println!("solve the exercises. Good luck!");
    println!();
    println!("The source for this exercise is in `exercises/intro/intro1.rs`. Have a look!");
    // 换行的字符串拼接（括号包裹，Rust 会自动拼接相邻字符串）
    println!(
        "Going forward, the source of the exercises will always be in the success/failure output."
    );
    println!();
    println!(
        "If you want to use rust-analyzer, Rust's LSP implementation, make sure your editor is set"
    );
    println!("up, and then run `rustlings lsp` before continuing.")
}