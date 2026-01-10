struct Solution;

use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};
impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        Self::dp_btree_map(nums)
    }
    fn dp_btree_map(nums: Vec<i32>) -> i32 {
        let mut dp = Vec::with_capacity(nums.len() / 8);
        for ii in (0..nums.len()).rev() {
            let partition =
                dp.partition_point(|pair: &(i32, BTreeMap<i32, usize>)| pair.0 > nums[ii]);
            let current_count = if partition == 0 {
                1
            } else {
                dp[partition - 1]
                    .1
                    .range((Excluded(nums[ii]), Unbounded))
                    .map(|pair| pair.1)
                    .sum()
            };
            if partition == dp.len() {
                dp.push((nums[ii], [(nums[ii], current_count)].into_iter().collect()));
            } else {
                dp[partition]
                    .1
                    .entry(nums[ii])
                    .and_modify(|count| {
                        *count += current_count;
                    })
                    .or_insert(current_count);
                dp[partition].0 = dp[partition].0.max(nums[ii]);
            }
        }
        dp.last().unwrap().1.values().sum::<usize>() as i32
    }
    fn dp_not_very_simple(nums: Vec<i32>) -> i32 {
        let mut dp_longest = vec![0; nums.len()];
        let mut dp_count = vec![1; nums.len()];
        let mut longest_length = 0;

        for ii in (0..nums.len()).rev() {
            let mut suffix_longest = 0;
            let mut count = 1;
            for jj in (ii + 1..nums.len()).rev() {
                if nums[ii] < nums[jj] {
                    if dp_longest[jj] > suffix_longest {
                        count = dp_count[jj];
                        suffix_longest = dp_longest[jj];
                    } else if dp_longest[jj] == suffix_longest {
                        count += dp_count[jj];
                    }
                }
            }
            dp_longest[ii] = 1 + suffix_longest;
            dp_count[ii] = count;
            longest_length = longest_length.max(dp_longest[ii]);
        }

        let mut result_count = 0;
        for ii in 0..nums.len() {
            if dp_longest[ii] == longest_length {
                result_count += dp_count[ii];
            }
        }
        result_count
    }
    fn dp(nums: Vec<i32>) -> i32 {
        let mut dp_longest = vec![0; nums.len() + 1];
        let mut dp_count = vec![1; nums.len() + 1];
        let mut longest_length = 0;

        for ii in (0..nums.len()).rev() {
            let mut last_index = nums.len();
            let mut count = dp_count[last_index];
            for jj in (ii + 1..nums.len()).rev() {
                if nums[ii] < nums[jj] {
                    if dp_longest[jj] > dp_longest[last_index] {
                        count = dp_count[jj];
                        last_index = jj;
                    } else if dp_longest[jj] == dp_longest[last_index] {
                        count += dp_count[jj];
                    }
                }
            }
            dp_longest[ii] = 1 + dp_longest[last_index];
            dp_count[ii] = count;
            longest_length = longest_length.max(dp_longest[ii]);
        }

        let mut result_count = 0;
        for ii in 0..nums.len() {
            if dp_longest[ii] == longest_length {
                result_count += dp_count[ii];
            }
        }
        result_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]), 5)
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::find_number_of_lis(vec![2, 1]), 2)
    }
}
