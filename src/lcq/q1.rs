struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        Self::implementation2(nums)
    }
    fn implementation1(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len() * 2);
        for _ in 0..2 {
            for element in nums.iter() {
                result.push(*element);
            }
        }
        result
    }
    fn implementation2(mut nums: Vec<i32>) -> Vec<i32> {
        for ii in 0..nums.len() {
            nums.push(nums[ii]);
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 3]),
            vec![1, 2, 3, 1, 2, 3]
        )
    }
}
