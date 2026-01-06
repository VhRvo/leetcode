struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![0; s.len() + 1];s.len() + 1];
        for ii in 0..s.len() + 1 {
            dp[0][ii] = 0;
            dp[1][ii] = 1;
        }
        for len in 2..s.len() + 1 {
            // ii + len <= s.len()
            // ii + len -1 < s.len()
            // len <= s.len() - ii
            // len < s.len() - ii + 1
            for ii in 0..s.len() - len + 1 {
                if s[ii] == s[ii + len - 1] {
                    dp[len][ii] = 2 + dp[len - 2][ii + 1];
                } else {
                    dp[len][ii] = dp[len - 1][ii + 1].max(dp[len - 1][ii])
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
        let s = "bbbab".to_string();
        let result = Solution::longest_palindrome_subseq(s);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let s = "cbbd".to_string();
        let result = Solution::longest_palindrome_subseq(s);
        assert_eq!(result, 2);
    }
}
