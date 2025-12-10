// move_semantics6.rs
fn main() {
    let data = "Rust is great!".to_string();

    // 移除 clone，传递引用（仅修改：添加 &）
    get_char(&data);

    string_uppercase(data);
}

// 关键修改：参数添加 &，改为接收不可变引用（不获取所有权）
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 无需修改参数（要求获取所有权），仅修复 to_uppercase 逻辑（该修改未违反“仅增减引用”的要求）
fn string_uppercase(mut data: String) {
    data = data.to_uppercase(); // 补充赋值，让大写生效
    println!("{}", data);
}