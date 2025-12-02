fn main() {
    call_me(3);
}

// 显式标注无返回值类型 ()，代码意图更清晰
fn call_me(num: u32) -> () {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}