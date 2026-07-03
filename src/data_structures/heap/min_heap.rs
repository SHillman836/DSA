use crate::data_structures::heap::node::HeapNode;
use std::vec::Vec;


/// This is the struct for a minimum heap. 
/// - In a minimum heap each child node has a value more 
///   than it's parent. 
/// - The smallest value is at the root node. 
/// - The left child can be found as `2 * parent index + 1`.
/// - The right child can be found as `2 * parent index + 2`.
/// - The parent can be found as `(child index - 1) / 2`.
#[derive(Debug, Clone, PartialEq)]
pub struct MinHeap {
    pub heap: Vec<HeapNode>
}

impl MinHeap {

    /// Recursive algorithm for removing values from the heap. 
    /// - Take the smaller child (or present child if a child isn't present)
    /// - Then if the smaller child is smaller than the target (the parent) then swap them
    pub fn re_heap_down(&mut self, target_index: usize) -> () {
        let left_child_index = 2 * target_index + 1;
        let right_child_index = 2 * target_index + 2;

        match (self.heap.get(left_child_index), self.heap.get(right_child_index)) {
            // There are 2 children
            (Some(left_child), Some(right_child)) => {
                // Swap the left child
                if left_child.value < right_child.value && left_child.value < self.heap[target_index].value {
                    self.heap.swap(left_child_index, target_index);
                    self.re_heap_down(left_child_index);
                
                // Swap the right child
                } else if right_child.value < left_child.value && right_child.value < self.heap[target_index].value {
                    self.heap.swap(right_child_index, target_index);
                    self.re_heap_down(right_child_index);

                // Nothing to swap we are done
                } else {
                    return;
                }
            },

            // There is only the left child
            (Some(left_child), None) => {
                // Left child needs to be swapped
                if left_child.value < self.heap[target_index].value {
                    self.heap.swap(left_child_index, target_index);
                    self.re_heap_down(left_child_index);
                } else {
                    return;
                }
            },

            // There is only the right child
            (None, Some(right_child)) => {
                // Right child needs to be swapped
                if right_child.value < self.heap[target_index].value {
                    self.heap.swap(right_child_index, target_index);
                    self.re_heap_down(right_child_index);
                } else {
                    return;
                }
            },

            // There is `None` values for both
            (None, None) => return,
        }
    }

    /// The removal method for the min heap.
    pub fn remove(&mut self) -> () {
        let last_index = self.heap.len() - 1;

        self.heap.swap(0, last_index);
        self.heap.pop();
        self.re_heap_down(0);
    }

    /// The recursive method for insertion.
    /// We follow the insert of the new value and then swap our way up the tree.
    /// If the parent is < the child we swap, which honors the key invariant of 
    /// a min heap in that parents are less than children. We don't need to guarantee
    /// anything else this is the only invariant required for a min heap. 
    pub fn re_heap_up(&mut self, target_index: usize) -> () {
        if target_index != 0 {
            let parent_index = (target_index - 1) / 2;
            if self.heap[target_index] < self.heap[parent_index] {
                self.heap.swap(target_index, parent_index);
                self.re_heap_up(parent_index);
            };
        }
    }

    /// The insertion method for the min heap. 
    pub fn insert(&mut self, value: i32) -> () {
        self.heap.push(HeapNode { value });
        let last_index = self.heap.len() - 1;
        self.re_heap_up(last_index);
    }

    /// The get method for the top value in the heap. This is guaranteed to be
    /// the lowest value overall in the heap.
    pub fn top_value(&self) -> i32 {
        return self.heap[0].value
    }

}



#[cfg(test)]
mod tests {
    use std::cmp::min;
    use super::*;

    #[test]
    fn test_remove() {
        let mut min_heap = MinHeap {
            heap: vec![
                HeapNode { value: 105 },
                HeapNode { value : 80 },
                HeapNode { value: 90 },
                HeapNode { value: 100 },
                HeapNode { value: 110 },
            ],
        };

        min_heap.re_heap_down(0);
        assert_eq!(min_heap.heap[0].value, 80);
        assert_eq!(min_heap.heap[1].value, 100);
        assert_eq!(min_heap.heap[2].value, 90);
        assert_eq!(min_heap.heap[3].value, 105);
        assert_eq!(min_heap.heap[4].value, 110);

        min_heap.remove();
        assert_eq!(min_heap.heap[0].value, 90);
        assert_eq!(min_heap.heap[1].value, 100);
        assert_eq!(min_heap.heap[2].value, 110);
        assert_eq!(min_heap.heap[3].value, 105);
    }

    #[test]
    fn test_insert() {
        let mut min_heap = MinHeap {
            heap: vec![
                HeapNode { value: 75 },
                HeapNode { value : 80 },
                HeapNode { value: 90 },
                HeapNode { value: 100 },
                HeapNode { value: 70 },
            ],
        };

        min_heap.re_heap_up(4);
        assert_eq!(min_heap.heap[0].value, 70);

        min_heap.insert(72);
        assert_eq!(min_heap.heap[2].value, 72);
    }

    #[test]
    fn test_top_value() {
        let mut min_heap = MinHeap {
            heap: vec![
                HeapNode { value: 70 },
                HeapNode { value : 80 },
                HeapNode { value: 90 },
                HeapNode { value: 100 },
                HeapNode { value: 110 },
            ],
        };

        let top_value = min_heap.top_value();
        assert_eq!(top_value, 70);
    }
}