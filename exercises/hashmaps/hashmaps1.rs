// hashmaps1.rs
use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // 1. 初始化一个空的 HashMap（可变的，因为要插入元素）
    let mut basket = HashMap::new();

    // 已有的香蕉
    basket.insert(String::from("banana"), 2);

    // 2. 添加至少两种更多的水果，确保总数≥5、种类≥3
    basket.insert(String::from("apple"), 2);    // 苹果：2个
    basket.insert(String::from("mango"), 2);    // 芒果：2个
    // 此时总数：2+2+2=6 ≥5，种类：3 ≥3，满足所有测试条件

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}