fn main() {
    // 创建空向量，vec0 拥有该向量的所有权
    let vec0 = Vec::new();

    // 将 vec0 的所有权移动到 fill_vec 函数，函数返回后所有权转移到 vec1
    let mut vec1 = fill_vec(vec0);

    // 打印 vec1 的初始状态（fill_vec 填充后）
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // 修改 vec1（因 vec1 是 mut 且拥有所有权，可正常修改）
    vec1.push(88);

    // 打印修改后的 vec1
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// 接收 Vec<i32> 的所有权，修改后返回（转移所有权）
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // 将传入的 vec 重新绑定为可变变量（获取所有权）
    let mut vec = vec;

    // 向向量中添加元素
    vec.push(22);
    vec.push(44);
    vec.push(66);

    // 返回向量，所有权从函数转移到调用方
    vec
}