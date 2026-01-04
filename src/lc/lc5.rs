struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        Self::dp_expand_around_center(s)
    }
    fn dp_implementation1(s: String) -> String {
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
    fn expand_around_center(s: &Vec<char>, mut left: isize, mut right: isize) -> (isize, isize) {
        let length = s.len() as isize;
        while left >= 0 && right < length && s[left as usize] == s[right as usize] {
            left -= 1;
            right += 1;
        }
        (left + 1, right)
    }
    fn dp_expand_around_center(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let mut start = 0;
        let mut end = 0;
        for ii in 0..s.len() {
            let ii = ii as isize;
            let len1 = Self::expand_around_center(&s, ii, ii);
            let len2 = Self::expand_around_center(&s, ii, ii + 1);
            for len in [len1, len2] {
                if len.1 - len.0 > end - start {
                    start = len.0;
                    end = len.1;
                }
            }
        }
        s[start as usize..end as usize].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }
}
