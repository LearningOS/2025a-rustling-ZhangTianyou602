// macros4.rs
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };  // 关键修改：添加分号分隔第一个模式
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };  // 关键修改：添加分号（最后一个模式也建议加，符合语法规范）
}

fn main() {
    my_macro!();
    my_macro!(7777);
}