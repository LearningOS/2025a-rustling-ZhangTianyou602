use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
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
        node.prev = self.end;
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

    pub fn reverse(&mut self) {
        // 边界处理：空链表或只有一个节点，无需反转
        if self.length <= 1 {
            return;
        }

        // 当前遍历的节点（从原头节点开始）
        let mut current = self.start;
        // 临时存储下一个节点（避免交换指针后丢失后续节点）
        let mut temp: Option<NonNull<Node<T>>>;

        // 遍历所有节点，逐个交换prev和next
        while let Some(mut curr_ptr) = current {
            unsafe {
                // 1. 保存当前节点的next（因为交换后会丢失）
                temp = (*curr_ptr.as_ptr()).next;

                // 2. 交换当前节点的prev和next
                (*curr_ptr.as_ptr()).next = (*curr_ptr.as_ptr()).prev;
                (*curr_ptr.as_ptr()).prev = temp;

                // 3. 移动到下一个节点（原prev，交换后变成next）
                current = temp;
            }
        }

        // 4. 交换链表的start和end（原头变尾，原尾变头）
        std::mem::swap(&mut self.start, &mut self.end);
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
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        for &num in &original_vec {
            list.add(num);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for (i, &expected) in reverse_vec.iter().enumerate() {
            assert_eq!(expected, *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        for &num in &original_vec {
            list.add(num);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for (i, &expected) in reverse_vec.iter().enumerate() {
            assert_eq!(expected, *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_empty_list() {
        let mut list = LinkedList::<i32>::new();
        list.reverse();
        assert_eq!(0, list.length);
        assert!(list.get(0).is_none());
    }

    #[test]
    fn test_reverse_single_node() {
        let mut list = LinkedList::<i32>::new();
        list.add(42);
        list.reverse();
        assert_eq!(1, list.length);
        assert_eq!(42, *list.get(0).unwrap());
    }
}