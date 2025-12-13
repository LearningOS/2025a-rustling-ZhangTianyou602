trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // 实现 append_bar 方法：追加 "Bar" 并返回自身
    fn append_bar(self) -> Self {
        // 先创建可变版本的字符串（self 是不可变的，需转换为 mut）
        let mut s = self;
        // 追加 "Bar" 到字符串末尾
        s.push_str("Bar");
        // 返回修改后的字符串
        s
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s); // 输出：s: FooBar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}