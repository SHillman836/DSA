use std::collections::HashMap;
use crate::data_structures::heap::min_heap::MinHeap;


/// Gets the top k frequent numbers using a heap. 
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // Initialise the hashmap storing counts.
    let mut k_counter = HashMap::new();

    // Loop through nums and add to the hashmap
    for num in nums.iter() {
        match k_counter.get_mut(num) {
            Some(count) => *count += 1,
            None => { 
                k_counter.insert(num, 1); 
            },
        }
    }

    // Instantiate min heap
    let mut min_heap = MinHeap { heap: Vec::new() };

    for (num, count) in k_counter.into_iter() {
        // Should really be unwrapped properly but we aren't handling errors
        let heap_length = i32::try_from(min_heap.heap.len()).unwrap();

        if heap_length < k {
            min_heap.insert(*num, count);
            continue;
        };

        if let Some(frequency) = min_heap.top_frequency() && count > frequency {
            min_heap.remove();
            min_heap.insert(*num, count);
        };
    };

    // Is unsorted but that isn't a requirement
    let mut final_vec = Vec::new();
    for node in min_heap.heap {
        final_vec.push(node.value_identity);
    }

    return final_vec;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm() {
        let input = vec![1, 1, 1, 2, 3, 2, 2, 4, 4, 5, 6, 10, 9];
        let output = top_k_frequent(input, 3);
        
        // Unsorted but that's OK
        assert_eq!(output[0], 4);
        assert_eq!(output[1], 2);
        assert_eq!(output[2], 1);
    }
}
