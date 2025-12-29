struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut hash_map = HashMap::new();
        for (index, num) in nums.into_iter().enumerate() {
            let index = index as i32;
            if let Some(jj) = hash_map.get(&(target - num)).copied() {
                return vec![index, jj];
            }
            hash_map.insert(num, index);
        }
        unreachable!()
    }
}
