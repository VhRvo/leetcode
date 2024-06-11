struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // let mut zero_position = 0;
        // let mut non_zero_position = 0;
        // while let Some(&value) = nums.get(non_zero_position) {
        //     if value != 0 {
        //         nums.swap(zero_position, non_zero_position);
        //         zero_position += 1;
        //     }
        //     non_zero_position += 1;
        // }
        let zero_position = nums.iter().position(|&num| num == 0);
        if let Some(mut zero_position) = zero_position {
            let mut non_zero_position = zero_position + 1;
            while let Some(value_ref) = nums.get_mut(non_zero_position) {
                let value = *value_ref;
                if value != 0 {
                    unsafe {
                        *value_ref = 0;
                        *nums.get_unchecked_mut(zero_position) = value;
                    }
                    // nums.swap(zero_position, non_zero_position);
                    zero_position += 1;
                }
                non_zero_position += 1;
            }
        }
    }
}
