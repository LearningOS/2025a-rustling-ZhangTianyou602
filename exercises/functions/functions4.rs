fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

/// 计算折后价格：偶数减10 Rustbucks，奇数减3 Rustbucks
fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

/// 判断数字是否为偶数
fn is_even(num: i32) -> bool {
    num % 2 == 0
}