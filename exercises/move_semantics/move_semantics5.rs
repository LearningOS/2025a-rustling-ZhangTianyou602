// move_semantics5.rs
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    // 关键重排：将 z 的定义和操作移到 y 的可变引用使用完毕后
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}