pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        // 调用 is_even 传入偶数（如4），断言结果为true
        assert!(is_even(4));
    }

    #[test]
    fn is_false_when_odd() {
        // 调用 is_even(5)，断言结果为false（用 assert!(!结果) 或 assert_eq!）
        assert!(!is_even(5));
        // 也可以用更清晰的 assert_eq!：assert_eq!(is_even(5), false);
    }
}