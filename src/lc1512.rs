use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold(
                (0, HashMap::<i32, usize>::new()),
                |(result, mut hash_map), &number| {
                    let entry = hash_map.entry(number).or_insert(0);
                    let result = result + *entry;
                    *entry += 1;
                    (result, hash_map)
                },
            )
            .0 as i32
    }
}
