/*
	heap
	This question requires you to implement a binary heap function
*/

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
        //TODO
        self.items.push(value);
        self.count += 1;

        let mut index = self.len();
        
        while index > 1 {
            let parent_index = self.parent_idx(index);
            if (self.comparator)(&(self.items[parent_index]), &(self.items[index])) {
                break;
            }
            self.items.swap(parent_index, index);
            index = parent_index;
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

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		self.left_child_idx(idx)
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
        //TODO
        if self.items.len() < 3 {
            return None;
        }
        let last_index = self.items.len() -1;
        self.items.swap(1, last_index);
        let value = self.items.pop()?;
        self.count -= 1;
        if !self.is_empty() {
            let mut index = 1;
            let length = self.items.len();
            
            while self.left_child_idx(index) < length {
                let mut left_child_index = self.left_child_idx(index);
                let right_child_index = self.right_child_idx(index);
                if right_child_index < length && (self.comparator)(&(self.items[right_child_index]), &(self.items[left_child_index])) {
                    left_child_index = right_child_index;
                }
                if (self.comparator)(&(self.items[index]), &(self.items[left_child_index])) {
                    break;
                }

                self.items.swap(index, left_child_index);
                index = left_child_index;
            }
        }
        Some(value)
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
        println!("min_heap {:?}", heap.items);
        heap.add(9);
        println!("min_heap {:?}", heap.items);
        heap.add(11);
        println!("min_heap {:?}", heap.items);
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
        println!("max_heap {:?}", heap.items);
        heap.add(9);
        println!("max_heap {:?}", heap.items);
        heap.add(11);
        println!("max_heap {:?}", heap.items);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}