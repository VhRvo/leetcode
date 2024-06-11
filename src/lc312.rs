struct Solution;

impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        let length = nums.len();
        nums.insert(0, 1);
        nums.push(1);
        let mut dp = vec![vec![0; nums.len()]; nums.len()];
        for ii in 1..=length {
            dp[ii - 1][ii + 1] = nums[ii - 1] * nums[ii] * nums[ii + 1];
        }
        for len in 2..=length {
            for start in 0..=length - len {
                let end = start + len + 1;
                for mid in start + 1..end {
                    dp[start][end] = dp[start][end]
                        .max(dp[start][mid] + dp[mid][end] + nums[start] * nums[end] * nums[mid]);
                }
            }
        }
        dp[0][length + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::max_coins(vec![1, 5]), 10);
    }
}
