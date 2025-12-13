#[derive(PartialEq, Debug)]
pub enum List {
    // 步骤1：用 Box 包裹递归的 List 类型，固定大小
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

// 步骤2：创建空的 cons 列表（直接返回 Nil）
pub fn create_empty_list() -> List {
    List::Nil
}

// 步骤2：创建非空的 cons 列表（示例：Cons(1, Cons(2, Nil))）
pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}