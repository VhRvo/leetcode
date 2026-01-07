struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut dp = vec![HashMap::new(); nums.len()];
        let mut result_longest = 0;
        /*
        dp[ii][nums[jj] - nums[ii]]
          -> max(dp[jj][nums[jj] - nums[ii]] + 1)
        */
        for ii in (0..nums.len()).rev() {
            let mut map = HashMap::new();
            for jj in (ii + 1..nums.len()).rev() {
                let difference = nums[jj] - nums[ii];
                let length = dp[jj].get(&difference).cloned().unwrap_or(1) + 1;
                /*
                map.entry(difference)
                    .and_modify(|e: &mut i32| {
                        *e = (*e).max(length);
                    })
                    .or_insert(length);
                */

                // If two element are the same, it is impossible the former one has a smaller length
                // So we can just insert/rewrite directly
                map.insert(difference, length);
                result_longest = result_longest.max(length)
            }
            dp[ii] = map;
        }
        result_longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_arith_seq_length(vec![3, 6, 9, 12]), 4)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_arith_seq_length(vec![9, 4, 7, 2, 10]), 3)
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]),
            4
        )
    }
}
