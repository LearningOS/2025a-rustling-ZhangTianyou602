use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    /// 合并两个有序单链表（默认升序），返回新的有序链表
    /// 前提：输入的两个链表必须是**升序排列**的
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
    where
        T: Ord, // 要求元素可比较（用于节点值大小判断）
    {
        let mut merged = LinkedList::new();
        // 双指针：分别指向 list_a 和 list_b 的当前节点
        let mut curr_a = list_a.start;
        let mut curr_b = list_b.start;

        // 循环比较两个链表的当前节点，按升序接入新链表
        while let (Some(mut ptr_a), Some(mut ptr_b)) = (curr_a, curr_b) {
            unsafe {
                // 比较当前两个节点的值
                if (*ptr_a.as_ptr()).val <= (*ptr_b.as_ptr()).val {
                    // 接入 list_a 的当前节点
                    merged.append_node(ptr_a);
                    // 移动 list_a 的指针到下一个节点
                    curr_a = (*ptr_a.as_ptr()).next;
                } else {
                    // 接入 list_b 的当前节点
                    merged.append_node(ptr_b);
                    // 移动 list_b 的指针到下一个节点
                    curr_b = (*ptr_b.as_ptr()).next;
                }
            }
        }

        // 处理 list_a 剩余的节点（如果有）
        while let Some(mut ptr_a) = curr_a {
            unsafe {
                merged.append_node(ptr_a);
                curr_a = (*ptr_a.as_ptr()).next;
            }
        }

        // 处理 list_b 剩余的节点（如果有）
        while let Some(mut ptr_b) = curr_b {
            unsafe {
                merged.append_node(ptr_b);
                curr_b = (*ptr_b.as_ptr()).next;
            }
        }

        merged
    }

    /// 辅助方法：将单个节点接入新链表尾部（维护 start 和 end）
    fn append_node(&mut self, node_ptr: NonNull<Node<T>>) {
        unsafe {
            // 断开节点原有的 next 指针（避免与原链表关联）
            (*node_ptr.as_ptr()).next = None;

            match self.end {
                None => {
                    // 新链表为空，当前节点既是头也是尾
                    self.start = Some(node_ptr);
                }
                Some(end_ptr) => {
                    // 新链表已有节点，将当前节点接入尾部
                    (*end_ptr.as_ptr()).next = Some(node_ptr);
                }
            }
            // 更新新链表的尾节点和长度
            self.end = Some(node_ptr);
            self.length += 1;
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

// 为 LinkedList 实现 Drop 特质，避免内存泄漏（关键！）
// 因为原链表的节点是用 Box::into_raw 转为原始指针的，需要手动释放
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.start;
        while let Some(mut ptr) = current {
            unsafe {
                // 移动到下一个节点
                current = (*ptr.as_ptr()).next;
                // 将原始指针转回 Box，自动释放内存
                let _ = Box::from_raw(ptr.as_ptr());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &num in &vec_a {
            list_a.add(num);
        }
        for &num in &vec_b {
            list_b.add(num);
        }
        println!("list a: {} | list b: {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List: {}", list_c);

        assert_eq!(target_vec.len() as u32, list_c.length);
        for (i, &expected) in target_vec.iter().enumerate() {
            assert_eq!(expected, *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &num in &vec_a {
            list_a.add(num);
        }
        for &num in &vec_b {
            list_b.add(num);
        }
        println!("list a: {} | list b: {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List: {}", list_c);

        assert_eq!(target_vec.len() as u32, list_c.length);
        for (i, &expected) in target_vec.iter().enumerate() {
            assert_eq!(expected, *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_empty_list() {
        // 测试一个空链表 + 非空链表
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_b = vec![5, 6, 7];
        let target_vec = vec![5, 6, 7];

        for &num in &vec_b {
            list_b.add(num);
        }
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        assert_eq!(target_vec.len() as u32, list_c.length);
        for (i, &expected) in target_vec.iter().enumerate() {
            assert_eq!(expected, *list_c.get(i as i32).unwrap());
        }

        // 测试两个空链表
        let list_a = LinkedList::<i32>::new();
        let list_b = LinkedList::<i32>::new();
        let list_c = LinkedList::<i32>::merge(list_a, list_b);
        assert_eq!(0, list_c.length);
        assert!(list_c.get(0).is_none());
    }

    #[test]
    fn test_merge_single_node() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        list_a.add(2);
        list_b.add(1);
        let target_vec = vec![1, 2];

        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        assert_eq!(target_vec.len() as u32, list_c.length);
        for (i, &expected) in target_vec.iter().enumerate() {
            assert_eq!(expected, *list_c.get(i as i32).unwrap());
        }
    }
}