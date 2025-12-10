// options1.rs
// 该函数根据时间返回冰箱里剩余的冰淇淋数量（Option<u16> 类型）
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // 处理时间超出 0-23 的情况，返回 None
    if time_of_day > 23 {
        None
    } else if time_of_day < 22 { // 22点前（10PM前），剩余5个
        Some(5)
    } else { // 22点及以后（10PM及以后），剩余0个
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // 修复：通过 unwrap() 获取 Option 内部的值（或使用 expect/unwrap_or 等）
        let icecreams = maybe_icecream(12).unwrap();
        assert_eq!(icecreams, 5);
    }
}