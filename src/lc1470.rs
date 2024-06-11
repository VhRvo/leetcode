struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let (front, rest) = nums.split_at(n as usize);
        front
            .iter()
            .zip(rest)
            .flat_map(|(&item1, &item2)| [item1, item2])
            .collect()
    }
}
