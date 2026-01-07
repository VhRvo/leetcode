struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1];
        let mut result = 0;
        for ii in (0..nums.len()).rev() {
            let mut last_element = i32::MAX;
            let mut last_index = nums.len();
            for jj in (ii + 1..nums.len()).rev() {
                if nums[ii] < nums[jj] {
                    if nums[jj] <= last_element && dp[jj] >= dp[last_index] {
                        last_index = jj;
                        last_element = nums[jj];
                    }
                }
            }
            dp[ii] = 1 + dp[last_index];
            result = result.max(dp[ii]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4)
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1)
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6)
    }
}
