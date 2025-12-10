pub fn capitalize_words_string(words: &[&str]) -> String {
    // 预计算总长度，分配足够内存
    let total_len: usize = words.iter().map(|s| s.len()).sum();
    let mut result = String::with_capacity(total_len);
    
    for word in words {
        result.push_str(&capitalize_first(word));
    }
    result
}