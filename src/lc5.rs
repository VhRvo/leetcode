struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let len = s.len();
        let mut dp = vec![vec![true; len]; 3];
        let mut result = s[0..1].iter().collect();
        for ll in 2..=len {
            let previous2 = (ll - 2) % 3;
            let current = ll % 3;
            for (ii, element) in s.iter().enumerate().take(len - ll + 1) {
                if *element == s[ii + ll - 1] && dp[previous2][ii + 1] {
                    dp[current][ii] = true;
                    result = s[ii..ii + ll].iter().collect();
                } else {
                    dp[current][ii] = false;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "aba");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }
}
