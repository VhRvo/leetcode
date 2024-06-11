struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        fn binary_search(arr: &[i32], left: usize, right: usize) -> usize {
            if left == right {
                left
            } else {
                let mid = left + (right - left) / 2;
                if arr[mid] > arr[mid + 1] {
                    binary_search(arr, left, mid)
                } else {
                    binary_search(arr, mid + 1, right)
                }
            }
        }
        binary_search(&arr, 0, arr.len() - 1) as i32
    }
}
