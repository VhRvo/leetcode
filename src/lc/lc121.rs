struct Solution;

use std::i32;
// native
// impl Solution {
//     pub fn max_profit(prices: Vec<i32>) -> i32 {
//         let transactions = 2;
//         let mut dp = vec![vec![0; transactions + 1]; prices.len() + 1];
//         for k in 0..=transactions {
//             dp[prices.len()][k] = 0;
//         }
//         for i in (0..prices.len()).rev() {
//             for k in 0..=transactions {
//                 dp[i][k] = dp[i + 1][k];
//                 if k > 0 && k % 2 == 0 {
//                     dp[i][k] = dp[i][k].max(dp[i + 1][k - 1] - prices[i]);
//                 }
//                 if k > 0 && k % 2 == 1 {
//                     dp[i][k] = dp[i][k].max(dp[i + 1][k - 1] + prices[i]);
//                 }
//             }
//         }
//         dp[0][transactions]
//     }
// }

// rolling array
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let transactions = 2;
        let mut dp = vec![0; transactions + 1];
        for k in 0..=transactions {
            dp[k] = 0;
        }
        for i in (0..prices.len()).rev() {
            for k in (0..=transactions).rev() {
                if k > 0 && k % 2 == 0 {
                    dp[k] = dp[k].max(dp[k - 1] - prices[i]);
                }
                if k > 0 && k % 2 == 1 {
                    dp[k] = dp[k].max(dp[k - 1] + prices[i]);
                }
            }
        }
        dp[transactions]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0)
    }
}
