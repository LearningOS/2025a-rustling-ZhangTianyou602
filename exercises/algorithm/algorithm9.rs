use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,          // 堆中有效元素个数
    items: Vec<T>,         // 存储堆元素（索引0占位，从1开始使用）
    comparator: fn(&T, &T) -> bool, // 比较器：true表示a应在b之前（满足堆序）
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 索引0占位，避免计算父子索引时出现0/2=0的问题
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // 插入元素到堆中
    pub fn add(&mut self, value: T) {
        // 1. 将新元素添加到向量尾部（保持完全二叉树结构）
        self.items.push(value);
        self.count += 1;

        // 2. 从尾部上浮调整堆序
        self.bubble_up(self.count);
    }

    // 上浮调整：从index位置向上，与父节点比较并交换，直到满足堆序
    fn bubble_up(&mut self, mut index: usize) {
        // 循环直到根节点（index=1）或满足堆序
        while index > 1 {
            let parent_idx = self.parent_idx(index);

            // 若当前节点与父节点满足堆序（comparator返回true），则调整结束
            if (self.comparator)(&self.items[index], &self.items[parent_idx]) {
                self.items.swap(index, parent_idx);
                index = parent_idx; // 继续向上调整
            } else {
                break; // 已满足堆序，退出
            }
        }
    }

    // 下沉调整：从index位置向下，与子节点比较并交换，直到满足堆序
    fn bubble_down(&mut self, mut index: usize) {
        // 循环直到无子节点或满足堆序
        while self.children_present(index) {
            // 找到符合堆序的子节点索引（最小堆找最小子节点，最大堆找最大子节点）
            let child_idx = self.smallest_child_idx(index);

            // 若当前节点与子节点满足堆序，则调整结束
            if (self.comparator)(&self.items[child_idx], &self.items[index]) {
                self.items.swap(index, child_idx);
                index = child_idx; // 继续向下调整
            } else {
                break; // 已满足堆序，退出
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    // 找到子节点中符合堆序的索引（根据comparator判断）
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        // 若只有左子节点，直接返回左子节点索引
        if right_idx > self.count {
            return left_idx;
        }

        // 比较左右子节点，返回符合堆序的索引（comparator返回true的那个）
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// 创建最小堆（父节点 ≤ 子节点）
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// 创建最大堆（父节点 ≥ 子节点）
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

// 实现迭代器：每次弹出堆顶元素（最小堆弹出最小值，最大堆弹出最大值）
impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // 1. 弹出堆顶元素（索引1）
        let top = self.items.swap_remove(1);
        self.count -= 1;

        // 2. 若堆仍有元素，从堆顶下沉调整堆序
        if !self.is_empty() {
            self.bubble_down(1);
        }

        Some(top)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2)); // 弹出最小值2
        assert_eq!(heap.next(), Some(4)); // 弹出次小值4
        assert_eq!(heap.next(), Some(9)); // 弹出次小值9
        heap.add(1); // 插入1，堆重新调整
        assert_eq!(heap.next(), Some(1)); // 弹出新的最小值1
        assert_eq!(heap.next(), Some(11)); // 弹出最后一个值11
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11)); // 弹出最大值11
        assert_eq!(heap.next(), Some(9)); // 弹出次大值9
        assert_eq!(heap.next(), Some(4)); // 弹出次大值4
        heap.add(1); // 插入1，堆重新调整
        assert_eq!(heap.next(), Some(2)); // 弹出次大值2
        assert_eq!(heap.next(), Some(1)); // 弹出最后一个值1
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_single_element() {
        let mut heap = MinHeap::new::<i32>();
        heap.add(5);
        assert_eq!(heap.len(), 1);
        assert_eq!(heap.next(), Some(5));
        assert_eq!(heap.is_empty(), true);
    }

    #[test]
    fn test_duplicate_elements() {
        let mut heap = MaxHeap::new::<i32>();
        heap.add(5);
        heap.add(3);
        heap.add(5);
        heap.add(2);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(5)); // 弹出第一个最大值5
        assert_eq!(heap.next(), Some(5)); // 弹出第二个最大值5
        assert_eq!(heap.next(), Some(3)); // 弹出次大值3
        assert_eq!(heap.next(), Some(2)); // 弹出最后一个值2
    }
}