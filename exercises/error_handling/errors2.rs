use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = match item_quantity.parse::<i32>() {
        Ok(num) => num,  // 解析成功，获取数量
        Err(e) => return Err(e),  // 解析失败，直接返回错误
    };

    Ok(qty * cost_per_item + processing_fee)
}