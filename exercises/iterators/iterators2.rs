// iterators2.rs
// Step 1. 实现首字母大写函数
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),                  // 空字符串直接返回空
        Some(first) => {
            // 首字母转大写 + 剩余字符拼接
            first.to_uppercase().to_string() + c.as_str()
        },
    }
}

// Step 2. 批量处理字符串切片，返回向量
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // 遍历每个单词，应用 capitalize_first，收集为 Vec<String>
    words.iter()
         .map(|&word| capitalize_first(word))
         .collect()
}

// Step 3. 批量处理并拼接为单个字符串
pub fn capitalize_words_string(words: &[&str]) -> String {
    // 先批量处理，再用 fold 拼接所有字符串
    capitalize_words_vector(words)
         .into_iter()
         .fold(String::new(), |acc, s| acc + &s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}