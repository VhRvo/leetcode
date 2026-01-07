struct Solution;

impl Solution {
    pub fn num_distinct(source: String, target: String) -> i32 {
        let source = source.chars().collect::<Vec<_>>();
        let target = target.chars().collect::<Vec<_>>();
        let mut dp = vec![0; target.len() + 1];
        dp[target.len()] = 1;
        /*
        dp(ii, jj)
          | source[ii] == target[jj]
          -> dp(ii + 1, jj + 1)
          : choose source[ii] and source[jj]
          -> dp(ii + 1, jj)
          : don't choose source[ii] and try next source character
        dp(ii, jj)
          | source[ii] != target[jj]
          -> dp(ii + 1, jj)
          : just try next source character
         */
        for ii in (0..source.len()).rev() {
            // 1d array: reverse the loop direction
            for jj in 0..target.len() {
                if source[ii] == target[jj] {
                    dp[jj] = dp[jj + 1] + dp[jj];
                }
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
            3
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::num_distinct("babgbag".to_string(), "bag".to_string()),
            5
        )
    }
}
