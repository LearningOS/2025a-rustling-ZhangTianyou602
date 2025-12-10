// macros3.rs
// 关键：给模块添加 #[macro_use] 注解，暴露内部定义的宏
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}