struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut hash_map = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            if let Some(&jj) = hash_map.get(&(target - num)) {
                return vec![index as i32, jj as i32];
            }
            hash_map.insert(num, index);
        }
        unreachable!()
    }
}
