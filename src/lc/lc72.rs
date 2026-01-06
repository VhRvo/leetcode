struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.chars().collect::<Vec<_>>();
        let word2 = word2.chars().collect::<Vec<_>>();

        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];

        for ii in 0..word1.len() {
            dp[ii][word2.len()] = (word1.len() - ii) as i32;
        }
        for jj in 0..word2.len() {
            dp[word1.len()][jj] = (word2.len() - jj) as i32;
        }
        for ii in (0..word1.len()).rev() {
            for jj in (0..word2.len()).rev() {
                if word1[ii] == word2[jj] {
                    dp[ii][jj] = dp[ii + 1][jj + 1];
                } else {
                    dp[ii][jj] = 1 + dp[ii + 1][jj].min(dp[ii][jj + 1]).min(dp[ii + 1][jj + 1]);
                }
            }
        }
        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_distance("horse".to_string(), "ros".to_string()), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_distance("intention".to_string(), "execution".to_string()), 5);
    }
}
