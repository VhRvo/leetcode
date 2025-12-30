struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        const ROUND: usize = 2;
        let mut dp = [[0; 2]; ROUND];
        dp[nums.len() % ROUND][0] = 0;
        dp[nums.len() % ROUND][1] = 0;
        for i in (0..nums.len()).rev() {
            let current = i % ROUND;
            let next = (i + 1) % ROUND;
            for able in 0..=1 {
                dp[current][able] = dp[next][1];
            }
            dp[current][1] = dp[current][1].max(dp[next][0] + nums[i]);
        }
        dp[0][1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
