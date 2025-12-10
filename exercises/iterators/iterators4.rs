// iterators4.rs
pub fn factorial(num: u64) -> u64 {
    // 核心逻辑：用迭代器的 range + fold 实现阶乘
    // 1. (1..=num) 生成 1 到 num 的闭区间迭代器（num=0 时迭代器为空）
    // 2. fold(1, |acc, x| acc * x) 从 1 开始累乘，空迭代器直接返回初始值 1
    (1..=num).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}