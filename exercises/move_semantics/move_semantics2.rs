// move_semantics2.rs
fn main() {
    // 第一步：先创建空向量，传入 fill_vec 后获取填充后的向量作为 vec0
    let vec0 = fill_vec(Vec::new());

    // 第二步：克隆 vec0 得到 vec1（保留 vec0 的所有权和内容）
    let mut vec1 = vec0.clone();

    // 输出 vec0（此时已有填充内容，符合预期）
    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

    // 修改 vec1
    vec1.push(88);

    // 输出 vec1（符合预期）
    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}