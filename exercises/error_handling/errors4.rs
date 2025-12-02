#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // 1. 先判断输入是否为负数
        if value < 0 {
            Err(CreationError::Negative)
        }
        // 2. 再判断输入是否为 0
        else if value == 0 {
            Err(CreationError::Zero)
        }
        // 3. 正数时，转换为 u64 并返回成功结果
        else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert!(PositiveNonzeroInteger::new(10).is_ok());
        assert_eq!(
            Err(CreationError::Negative),
            PositiveNonzeroInteger::new(-10)
        );
        assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
    }
}