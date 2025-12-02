use std::num::ParseIntError;

// 1. 将 main 函数返回类型改为 Result<(), ParseIntError>
// () 表示成功时无返回值，ParseIntError 是可能的错误类型
fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // 2. 现在可以使用 ? 运算符，解析失败会直接返回错误
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    // 3. 成功执行时，返回 Ok(())
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}