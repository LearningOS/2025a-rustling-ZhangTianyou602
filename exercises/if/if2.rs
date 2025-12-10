// if2.rs
pub fn foo_if_fizz(fizzish: &str) -> &str {
    // 条件分支逻辑：匹配不同输入返回对应字符串
    if fizzish == "fizz" {
        "foo"          // 输入"fizz"返回"foo"
    } else if fizzish == "fuzz" {
        "bar"          // 输入"fuzz"返回"bar"
    } else {
        "baz"          // 其他所有输入返回"baz"
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}