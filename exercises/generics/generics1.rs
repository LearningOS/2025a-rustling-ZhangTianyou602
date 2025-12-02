fn main() {
    // 明确 Vec 存储的类型是 &str（字符串切片），匹配 push 的 "milk" 类型
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}