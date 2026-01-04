struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let mut next_odd = vec![0; nums.len()];
        let mut next_even = vec![0; nums.len()];
        let mut last_even = nums.len();
        let mut last_odd = nums.len();
        for ii in (0..nums.len()).rev() {
            next_odd[ii] = last_odd;
            next_even[ii] = last_even;
            if nums[ii] % 2 == 0 {
                last_even = ii;
            } else {
                last_odd = ii;
            }
        }

        let x = x as i64;
        let mut dp = vec![i64::MIN / 2; nums.len()];
        dp[0] = nums[0] as i64;
        let mut result = i64::MIN;
        for ii in 0..nums.len() {
            let next_even = next_even[ii];
            let next_odd = next_odd[ii];
            if next_even == nums.len() {
                result = result.max(dp[ii]);
            } else {
                let correction = if nums[ii] % 2 == 0 { 0 } else { x };
                dp[next_even] = dp[next_even].max(dp[ii] + nums[next_even] as i64 - correction);
            }
            if next_odd == nums.len() {
                result = result.max(dp[ii]);
            } else {
                let correction = if nums[ii] % 2 == 0 { x } else { 0 };
                dp[next_odd] = dp[next_odd].max(dp[ii] + nums[next_odd] as i64 - correction);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::max_score(vec![2, 3, 6, 1, 9, 2], 5), 13);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::max_score(vec![2, 4, 6, 8], 3), 20);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::max_score(vec![2, 4, 6, 8], 8), 20);
    }
}
