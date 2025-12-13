fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // 普通数组
    let v = vec![10, 20, 30, 40]; // 用 vec! 宏创建与数组元素相同的向量

    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]); // 向量切片与数组比较
    }
}