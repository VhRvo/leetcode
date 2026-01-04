struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero_index = 0;
        for ii in 0..nums.len() {
            if nums[ii] != 0 {
                nums.swap(zero_index, ii);
                zero_index += 1;
            }
        }
        // let zero_index = nums.iter().copied().position(|num| num == 0);
        // if let Some(mut zero_index) = zero_index {
        //     loop {
        //         let mut non_zero_index = zero_index + 1;
        //         while non_zero_index < nums.len() && nums[non_zero_index] == 0 {
        //             non_zero_index += 1;
        //         }
        //         if non_zero_index == nums.len() {
        //             break;
        //         }
        //         nums.swap(zero_index, non_zero_index);
        //         while zero_index < nums.len() && nums[zero_index] != 0 {
        //             zero_index += 1;
        //         }
        //         if zero_index == nums.len() {
        //             break;
        //         }
        //     }
        // }
        // let mut zero_position = 0;
        // let mut non_zero_position = 0;
        // while let Some(&value) = nums.get(non_zero_position) {
        //     if value != 0 {
        //         nums.swap(zero_position, non_zero_position);
        //         zero_position += 1;
        //     }
        //     non_zero_position += 1;
        // }

        // let zero_position = nums.iter().position(|&num| num == 0);
        // if let Some(mut zero_position) = zero_position {
        //     let mut non_zero_position = zero_position + 1;
        //     while let Some(value_ref) = nums.get_mut(non_zero_position) {
        //         let value = *value_ref;
        //         if value != 0 {
        //             unsafe {
        //                 *value_ref = 0;
        //                 *nums.get_unchecked_mut(zero_position) = value;
        //             }
        //             // nums.swap(zero_position, non_zero_position);
        //             zero_position += 1;
        //         }
        //         non_zero_position += 1;
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, [1, 3, 12, 0, 0]);
    }
    #[test]
    fn test2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, [0]);
    }
}
