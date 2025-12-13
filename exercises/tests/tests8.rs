// build.rs
fn main() {
    // 向编译环境注入 feature = "pass" 的配置
    println!("cargo:rustc_cfg=feature=\"pass\"");
}