struct Solution;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp_starting_at = vec![vec![0; nums2.len() + 1]; 2];
        for ii in (0..nums1.len()).rev() {
            let current = ii % 2;
            let next = (ii + 1) % 2;
            for jj in (0..nums2.len()).rev() {
                if nums1[ii] == nums2[jj] {
                    dp_starting_at[current][jj] = dp_starting_at[next][jj + 1] + 1;
                } else {
                    dp_starting_at[current][jj] =
                        dp_starting_at[current][jj + 1].max(dp_starting_at[next][jj]);
                }
            }
        }
        dp_starting_at[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]),
            2
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
            3
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
            2
        )
    }
}
