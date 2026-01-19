struct Solution;

use std::i32;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        Self::dp_rolling_array2(prices)
    }
    fn dp_native2(prices: Vec<i32>) -> i32 {
        // dp_starting_at[index][colddown][hold]
        let mut dp_starting_at = vec![vec![vec![0; 2]; 2]; prices.len() + 1];
        for ii in (0..prices.len()).rev() {
            let current = ii;
            let next = ii + 1;
            dp_starting_at[current][0][0] =
                dp_starting_at[next][0][0].max(dp_starting_at[next][0][1] - prices[ii]);
            dp_starting_at[current][0][1] =
                dp_starting_at[next][0][1].max(dp_starting_at[next][1][0] + prices[ii]);
            dp_starting_at[current][1][0] = dp_starting_at[next][0][0];
        }
        dp_starting_at[0][0][0]
    }
    fn dp_rolling_array2(prices: Vec<i32>) -> i32 {
        // dp_starting_at[index][colddown][hold]
        // let mut dp_starting_at = vec![vec![vec![0; 2]; 2]; 2];
        let mut dp_starting_at = [[[0; 2]; 2]; 2];
        for (ii, price) in prices.iter().enumerate().rev() {
            // Use current and next only for indexing the 2-row rolling array
            let current = ii % 2;
            let next = (ii + 1) % 2;
            // let dp_starting_at[current] = &mut dp_starting_at[ii % 2];
            // let dp_at_next = &mut dp_starting_at[(ii + 1) % 2];
            // Important: index prices with `ii`; using `current` (mod 2) desyncs rows and is buggy
            dp_starting_at[current][0][0] =
                dp_starting_at[next][0][0].max(dp_starting_at[next][0][1] - price);
            dp_starting_at[current][0][1] =
                dp_starting_at[next][0][1].max(dp_starting_at[next][1][0] + price);
            dp_starting_at[current][1][0] = dp_starting_at[next][0][0];
        }
        dp_starting_at[0][0][0]
    }
    fn dp_native(prices: Vec<i32>) -> i32 {
        let mut dp = vec![[[0; 2]; 2]; prices.len() + 1];
        dp[prices.len()][1] = [i32::MIN / 2; 2];
        for i in (0..prices.len()).rev() {
            for hold in 0..2 {
                for cooldown in 0..2 {
                    dp[i][hold][cooldown] = dp[i + 1][hold][0];
                }
            }
            dp[i][0][0] = dp[i + 1][0][0].max(dp[i + 1][1][0] - prices[i]);
            for cooldown in 0..2 {
                dp[i][1][cooldown] = dp[i + 1][1][cooldown].max(dp[i + 1][0][1] + prices[i]);
            }
        }
        dp[0][0][0]
    }
    fn dp_rolling_array(prices: Vec<i32>) -> i32 {
        let mut dp = [[[0; 2]; 2]; 2];
        dp[prices.len() % 2][1] = [i32::MIN / 2; 2];
        for i in (0..prices.len()).rev() {
            let current = i % 2;
            let next = (i + 1) % 2;
            for hold in 0..2 {
                for cooldown in 0..2 {
                    dp[current][hold][cooldown] = dp[next][hold][0];
                }
            }
            dp[current][0][0] = dp[next][0][0].max(dp[next][1][0] - prices[i]);
            for cooldown in 0..2 {
                dp[current][1][cooldown] = dp[next][1][cooldown].max(dp[next][0][1] + prices[i])
            }
        }
        dp[0][0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::max_profit(vec![1]), 0)
    }
}
