struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.as_bytes().to_vec();
        let text2 = text2.as_bytes().to_vec();
        let mut dp_starting_at = vec![vec![0; text2.len() + 1]; 2];
        for ii in (0..text1.len()).rev() {
            let current = ii % 2;
            let next = (ii + 1) % 2;
            for jj in (0..text2.len()).rev() {
                if text1[ii] == text2[jj] {
                    dp_starting_at[current][jj] = dp_starting_at[next][jj + 1] + 1;
                } else {
                    dp_starting_at[current][jj] =
                        (dp_starting_at[current][jj + 1]).max(dp_starting_at[next][jj]);
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
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "def".to_string()),
            0
        );
    }
}
