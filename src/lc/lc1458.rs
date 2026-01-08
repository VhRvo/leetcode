struct Solution;

use std::i32;
impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let choices = 2;
        let mut dp = vec![vec![vec![0; choices]; nums2.len() + 1]; nums1.len() + 1];
        for count in 1..choices {
            dp[nums1.len()][nums2.len()][count] = i32::MIN / 2;
            for jj in 0..nums2.len() {
                dp[nums1.len()][jj][count] = i32::MIN / 2;
            }
            for ii in 0..nums1.len() {
                dp[ii][nums2.len()][count] = i32::MIN / 2;
            }
        }

        for ii in (0..nums1.len()).rev() {
            for jj in (0..nums2.len()).rev() {
                for count in 0..choices {
                    let pre_count = if count == 0 { 0 } else { count - 1 };
                    let both = nums1[ii] * nums2[jj];
                    if (nums1[ii].is_positive() && nums2[jj].is_positive())
                        || (nums1[ii].is_negative() && nums2[jj].is_negative())
                    {
                        dp[ii][jj][count] = (both + dp[ii + 1][jj + 1][pre_count])
                            .max(dp[ii + 1][jj][count])
                            .max(dp[ii][jj + 1][count]);
                    } else {
                        dp[ii][jj][count] = (both + dp[ii + 1][jj + 1][pre_count])
                            .max(dp[ii + 1][jj + 1][count])
                            .max(dp[ii + 1][jj][count])
                            .max(dp[ii][jj + 1][count]);
                    }
                }
            }
        }
        dp[0][0][choices - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_dot_product(vec![2, 1, -2, 5], vec![3, 0, -6]),
            18
        );
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::max_dot_product(vec![3, -2], vec![2, -6, 7]), 21);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::max_dot_product(vec![-1, -1], vec![1, 1]), -1);
    }
}
