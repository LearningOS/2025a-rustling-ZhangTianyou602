pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // 空名字时返回 Err，包含具体的错误说明
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        // 非空名字时返回 Ok，包含生成的标签文本
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}