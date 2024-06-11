struct Solution;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let length = nums.len();
        let k = k as usize % length;
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}
