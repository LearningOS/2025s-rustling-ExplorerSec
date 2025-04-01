/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM DONE

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // 可以假设原堆是有序的
        if self.count == 0{
            // 一度以为自己算法有问题，最后经过调试发现
            // 内部的 item[0] 被已经给出的代码框架初始化为了 0
            // 这个真的绷不住
            // 所以这个 if 是给它打的补丁
            self.items[0] = value; 
        } else {
            self.items.push(value);
        }
        let mut idx_up =self.count;
        while (self.comparator)(&self.items[idx_up],&self.items[idx_up/2]){
            self.items.swap(idx_up,idx_up/2);
            idx_up = idx_up/2;

            let mut idx_down = idx_up;
            loop{
                let mut idx_child = self.smallest_child_idx(idx_down);
                if idx_child > 0 && (self.comparator)(&self.items[idx_child],&self.items[idx_down]){
                    self.items.swap(idx_down,idx_child);
                    idx_down = idx_child;
                }else{
                    break;
                }
            }


            
        }
        self.count +=1;
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

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if idx*2 >= self.count{
            0
        }else if idx*2+1 < self.count && (self.comparator)(&self.items[idx*2+1],&self.items[idx*2]){
            idx*2+1
        }else{
            idx*2
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            None
        }else{
            self.count -=1;
            self.items.swap(0,self.count);
            let val = self.items.pop();
            let mut idx = 0;
            loop{
                let mut idx_child = self.smallest_child_idx(idx);
                if idx_child > 0 && (self.comparator)(&self.items[idx_child],&self.items[idx]){
                    self.items.swap(idx,idx_child);
                    idx = idx_child;
                }else{
                    break;
                }
            }
            val
        }
        
		
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
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}