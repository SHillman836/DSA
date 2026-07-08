

/// This is the recursive function of the merge sort algorithm.
pub fn merge_sort(nums: &[i32]) -> Vec<i32> {
    // This is the base case at the bottom of the whole recursive stack 
    // and is triggered by continual `merge_sort` calls. 
    if nums.len() == 1 {
        return nums.to_vec();
    };

    // We split a slice - this is the divide part of the algorithm. 
    let mid = nums.len() / 2;
    let (left_slice, right_slice) = nums.split_at(mid);
    
    // We then recursively call `merge_sort` and these recursive calls continue
    // until we hit the base case of len == 1
    // We need to store these slices in variables so we're merging the prior merged
    // vecs on the way back up the call stack
    let left_slice = merge_sort(left_slice);
    let right_slice = merge_sort(right_slice);

    // This is the conquer part of the algo and will be called for every function
    // in the call stack on the way up after we've hitten that base case and then each
    // of these functions return the merged (and ordered) slice back up to the next function
    return merge(&left_slice[..], &right_slice[..])
}


/// This function merges and orders two slices. 
pub fn merge(left_slice: &[i32], right_slice: &[i32]) -> Vec<i32> {

    let mut left_slice_pointer: usize = 0;
    let mut right_slice_pointer: usize = 0;
    let mut merged_vec: Vec<i32> = Vec::new();

    let left_slice_end: usize = left_slice.len() - 1;
    let right_slice_end: usize = right_slice.len() - 1;

    while left_slice_pointer <= left_slice_end || right_slice_pointer <= right_slice_end {
        match (left_slice.get(left_slice_pointer), right_slice.get(right_slice_pointer)) {
            (Some(left_slice_val), Some(right_slice_val)) => {
                if left_slice_val <= right_slice_val {
                    merged_vec.push(*left_slice_val);
                    left_slice_pointer += 1;
                } else {
                    merged_vec.push(*right_slice_val);
                    right_slice_pointer += 1;
                };
            },
            (Some(left_slice_val), None) => {
                merged_vec.push(*left_slice_val);
                left_slice_pointer += 1;
            },
            (None, Some(right_slice_val)) => {
                merged_vec.push(*right_slice_val);
                right_slice_pointer += 1;
            },
            (None, None) => panic!("Should never happen"),
        };
    };

    return merged_vec;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_test() {
        // [0..=2] slice 1
        // [3..=5] slice 2
        let vec = vec![1, 5, 7, 4, 7, 9];

        let result = merge(&vec[0..=2], &vec[3..=5]);

        assert_eq!(result.len(), 6);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 4);
        assert_eq!(result[2], 5);
        assert_eq!(result[3], 7);
        assert_eq!(result[4], 7);
        assert_eq!(result[5], 9);
    }

    #[test]
    fn test_merge_sort() {
        let mut nums = vec![0, 4, 7, 4, 3, 2, 3, 4, 5];
        let high = nums.len() - 1;
        let result = merge_sort(&mut nums[0..=high]);

        assert_eq!(result[0], 0);
        assert_eq!(result[1], 2);
        assert_eq!(result[2], 3);
        assert_eq!(result[3], 3);
        assert_eq!(result[4], 4);
        assert_eq!(result[5], 4);
        assert_eq!(result[6], 4);
        assert_eq!(result[7], 5);
        assert_eq!(result[8], 7);
    }
}