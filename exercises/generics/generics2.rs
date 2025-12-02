// 定义泛型结构体 Wrapper<T>，T 是类型参数，代表可以存储的任意类型
struct Wrapper<T> {
    value: T, // value 的类型由 T 决定，与结构体的泛型参数一致
}

// 为泛型结构体实现方法，impl 后需要带上 <T> 声明泛型参数
impl<T> Wrapper<T> {
    // 构造函数的参数类型为 T，返回 Self（即 Wrapper<T>）
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}