struct Solution;

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let value_difference = value_difference as u32;
        let (mut max_value, mut max_pos) = (i32::MIN, 0);
        let (mut min_value, mut min_pos) = (i32::MAX, 0);
        for (index, (front_value, elem)) in nums
            .iter()
            .copied()
            .zip(nums.iter().skip(index_difference as usize).copied())
            .enumerate()
        {
            if front_value > max_value {
                max_value = front_value;
                max_pos = index;
            }
            if front_value < min_value {
                min_value = front_value;
                min_pos = index;
            }
            if max_value.abs_diff(elem) >= value_difference {
                return vec![max_pos as i32, index as i32 + index_difference];
            }
            if min_value.abs_diff(elem) >= value_difference {
                return vec![min_pos as i32, index as i32 + index_difference];
            }
        }
        vec![-1, -1]
    }
}
