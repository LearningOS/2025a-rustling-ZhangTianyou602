// 移除不必要的 allow 注解（修复后无未使用变量/赋值）
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // 修复：删除对 None 的 unwrap() 调用（避免 panic）
    }

    let my_arr = &[
        -1, -2, -3, // 修复：添加缺失的逗号
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 修复：先创建向量，再原地 resize（不接收返回值）
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.resize(0, 5); // resize(0, 5) 清空向量
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 修复：使用 std::mem::swap 正确交换变量
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}