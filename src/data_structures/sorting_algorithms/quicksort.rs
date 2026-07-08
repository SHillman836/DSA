

/// This is the partition function which is called recursively.
/// - nums: The mutable slice into the vector we're sorting.
/// - low: The index at the start of the period in the slice we're sorting.
/// - high: The pivot and the end of the period in the slice we're sorting.
pub fn partition(
    nums: &mut [i32],
    low: usize,
    high: usize
) -> usize {
    // Swap pivot index with a number in the middle for random pivots
    let pivot_index = low + (high - low) / 2;
    nums.swap(pivot_index, high);

    // The pointer which indicates the place to swap next
    let mut next_to_swap: usize = low;

    // We don't go up to the pivot at that point we end and swap the pivot accordingly
    for index in low..high {
        if nums[index] < nums[high] {
            nums.swap(index, next_to_swap);
            next_to_swap += 1;
        };
    };

    // We then swap the pivot with the next_to_swap value and return the index of the new pivot positioned in the `next_to_swap` field
    nums.swap(next_to_swap, high);

    return next_to_swap;
}

pub fn recursive_quick_sort(
    nums: &mut [i32],
    low: usize,
    high: usize
) -> () {
    if low < high {
        let pivot_index = partition(nums, low, high);

        if pivot_index > 0 {
            recursive_quick_sort(nums, low, pivot_index - 1);
        };
        recursive_quick_sort(nums, pivot_index + 1, high);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut nums = vec![0, 4, 7, 4, 3, 2, 3, 4, 5];
        let mut high = nums.len() - 1;
        recursive_quick_sort(&mut nums[0..=high], 0, high);

        assert_eq!(nums[0], 0);
        assert_eq!(nums[1], 2);
        assert_eq!(nums[2], 3);
        assert_eq!(nums[3], 3);
        assert_eq!(nums[4], 4);
        assert_eq!(nums[5], 4);
        assert_eq!(nums[6], 4);
        assert_eq!(nums[7], 5);
        assert_eq!(nums[8], 7);
    }
}