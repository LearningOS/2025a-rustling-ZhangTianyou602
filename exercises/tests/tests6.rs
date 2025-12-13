struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: 遵循函数安全契约，ptr 是由 Box::into_raw 生成的合法裸指针
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    // 修改 b 字段为预期值，满足测试断言
    ret.b = Some("hello".to_owned());
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None });

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: 传入的是 Box::into_raw 生成的裸指针，符合函数安全契约
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        // 验证内存地址一致（还原后的 Box 指向原内存）
        assert!(ptr_1 == ptr_2);
        // 验证 b 字段被正确修改
        assert!(ret.b == Some("hello".to_owned()));
    }
}