struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut dp = vec![0; arr.len() + 1];
        let mut longest_length = 0;
        let mut map = HashMap::new();

        for ii in (0..arr.len()).rev() {
            let mut last_index = arr.len();
            if let Some(&jj) = map.get(&(arr[ii] + difference)) {
                if dp[jj] > dp[last_index] {
                    last_index = jj;
                }
            }
            dp[ii] = 1 + dp[last_index];
            longest_length = longest_length.max(dp[ii]);
            map.entry(arr[ii])
                .and_modify(|index| {
                    if dp[*index] < dp[ii] {
                        *index = ii
                    }
                })
                .or_insert(ii);
        }

        longest_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_subsequence(vec![1, 2, 3, 4], 1), 4)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_subsequence(vec![1, 3, 5, 7], 1), 1)
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2),
            4
        )
    }
}
