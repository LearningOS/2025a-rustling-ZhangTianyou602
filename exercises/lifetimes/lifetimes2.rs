// lifetimes2.rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        // 关键修改：将 result 移到 string2 的作用域内，保证引用有效期匹配
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }
}