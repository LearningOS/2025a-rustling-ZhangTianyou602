// macros2.rs
// 关键修改：将宏定义移到调用之前（Rust 要求宏必须先定义后使用）
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}