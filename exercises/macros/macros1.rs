// macros1.rs
// 移除 I AM NOT DONE 注释后即可正常运行
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro();
}