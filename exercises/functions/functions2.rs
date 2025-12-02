fn main() {
    call_me(3);
}

// 显式标注返回类型为 ()（无返回值），让代码更清晰规范
fn call_me(num: i32) -> () {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}