#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T> {
    q1: Queue<T>, // 主队列：存储栈中所有有效元素
    q2: Queue<T>, // 辅助队列：临时迁移元素
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }

    // 入栈：直接加入主队列 q1（新元素在队尾，对应栈顶）
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }

    // 出栈：弹出栈顶元素（q1 的最后一个元素）
    pub fn pop(&mut self) -> Result<T, &str> {
        // 若主队列空，栈空返回错误
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        // 步骤1：将 q1 中除最后一个元素外的所有元素迁移到 q2
        while self.q1.size() > 1 {
            // q1 非空且size>1，dequeue 必然成功，unwrap 安全
            let temp = self.q1.dequeue().unwrap();
            self.q2.enqueue(temp);
        }

        // 步骤2：弹出 q1 中剩余的最后一个元素（栈顶）
        let top = self.q1.dequeue().unwrap();

        // 步骤3：交换 q1 和 q2，让 q2 成为新的主队列（q1 清空备用）
        std::mem::swap(&mut self.q1, &mut self.q2);

        Ok(top)
    }

    // 判空：主队列 q1 为空则栈空
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }

    // 额外测试：单元素栈
    #[test]
    fn test_single_element() {
        let mut s = myStack::<i32>::new();
        s.push(42);
        assert_eq!(s.pop(), Ok(42));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert!(s.is_empty());
    }

    // 额外测试：交替入栈出栈
    #[test]
    fn test_push_pop_interleaved() {
        let mut s = myStack::<String>::new();
        s.push("a".to_string());
        s.push("b".to_string());
        assert_eq!(s.pop(), Ok("b".to_string()));
        s.push("c".to_string());
        assert_eq!(s.pop(), Ok("c".to_string()));
        assert_eq!(s.pop(), Ok("a".to_string()));
        assert!(s.is_empty());
    }
}