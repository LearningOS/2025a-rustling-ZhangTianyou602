// move_semantics4.rs
fn main() {
    // 移除了原有的 vec0 定义，直接调用 fill_vec() 获取向量
    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// 重构后：不再接收 Vec<i32> 参数，而是内部创建向量并返回
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new(); // 向量在函数内创建

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec // 返回向量，所有权转移到 main 中的 vec1
}