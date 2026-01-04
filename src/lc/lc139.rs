struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_set = word_dict.into_iter().collect::<HashSet<_>>();
        let mut dp = vec![vec![false; s.len() + 1]; s.len() + 1];
        dp[0] = vec![true; s.len() + 1];
        for len in 1..s.len() + 1 {
            // ii + len <= s.len()
            // ii <= s.len() - len
            for ii in 0..s.len() - len + 1 {
                // dp[ii, len] -> contains[ii, jj) && dp[jj, len - (jj - ii)]
                // 0 <= len - (jj - ii) < len
                // ii < jj && jj <= ii + len
                for jj in ii + 1..ii + len + 1 {
                    dp[len][ii] = dp[len][ii] || (word_set.contains(&s[ii..jj]) && dp[len - (jj - ii)][jj]);
                }
            }
        }
        dp[s.len()][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::word_break(
                "leetcode".to_owned(),
                ["leet", "code"].map(|s| s.to_owned()).to_vec()
            ),
            true
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::word_break(
                "applepenapple".to_owned(),
                ["apple", "pen"].map(|s| s.to_owned()).to_vec()
            ),
            true
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::word_break(
                "catsandog".to_owned(),
                ["cats", "dog", "sand", "and", "cat"]
                    .map(|s| s.to_owned())
                    .to_vec()
            ),
            false
        )
    }
}
