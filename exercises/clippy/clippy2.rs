fn main() {
    let mut res = 42;
    let option = Some(12);
    
    // 明确匹配 Option 的 Some 变体，处理内部值
    if let Some(x) = option {
        res += x;
    }
    
    println!("{}", res);
}