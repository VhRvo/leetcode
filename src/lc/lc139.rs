struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word_set = word_dict.into_iter().collect::<HashSet<_>>();
        let mut dp = vec![false; s.len() + 1];
        // the length of dp[0] is s.len() + 1
        dp[s.len()] = true;
        // very critical!
        // dp(ii) -> contains[ii, jj) && dp(jj)
        for ii in (0..s.len() + 1).rev() {
            for jj in ii + 1..s.len() + 1 {
                dp[ii] = dp[ii] || (word_set.contains(&s[ii..jj]) && dp[jj]);
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
