struct Solution;

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        for ii in (0..nums1.len()).rev() {
            for jj in (0..nums2.len()).rev() {
                if (nums1[ii].is_positive() && nums2[jj].is_positive())
                    || (nums1[ii].is_negative() && nums2[jj].is_negative())
                {
                    dp[ii][jj] = (nums1[ii] * nums2[jj] + dp[ii + 1][jj + 1])
                        .max(dp[ii + 1][jj])
                        .max(dp[ii][jj + 1]);
                } else {
                    dp[ii][jj] = dp[ii + 1][jj + 1].max(dp[ii + 1][jj]).max(dp[ii][jj + 1]);
                }
            }
        }
        let mut has_zero = false;
        let mut mininal_non_zero_absolutes = Vec::with_capacity(2);
        for nums in [nums1, nums2] {
            let mut mininal_non_zero_absolute = i32::MAX;
            for ii in 0..nums.len() {
                if nums[ii] == 0 {
                    has_zero = true;
                    break;
                }
                mininal_non_zero_absolute =
                    mininal_non_zero_absolute.min(nums[ii].abs());
            }
            mininal_non_zero_absolutes.push(mininal_non_zero_absolute);
        }
        if has_zero {
            dp[0][0]
        } else if dp[0][0] == 0{
            - mininal_non_zero_absolutes.iter().product::<i32>()
        } else {
            dp[0][0]
        }
    }
}

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
