struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        Self::one(s)
    }
    fn one(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<_>>();
        let mut current = 0;
        let mut dp_staring_at = vec![vec![0; chars.len() + 1]; 2];
        // ii iterates backward over the reversed view of chars
        for ii in 0..chars.len() {
            current = ii % 2;
            let next = (ii + 1) % 2;
            for jj in (0..chars.len()).rev() {
                if chars[ii] == chars[jj] {
                    dp_staring_at[current][jj] = dp_staring_at[next][jj + 1] + 1;
                } else {
                    dp_staring_at[current][jj] =
                        dp_staring_at[current][jj + 1].max(dp_staring_at[next][jj]);
                }
            }
        }
        (chars.len() - dp_staring_at[current][0]) as i32
    }
    fn longest_common_subsequence(chars1: &[char], chars2: &[char]) -> i32 {
        let total = chars1.len() + chars2.len();
        let mut dp_staring_at = vec![vec![0; chars2.len() + 1]; 2];
        // chars1 is a reversed view of the underlying char sequence
        let mut current = 0;
        for ii in 0..chars1.len() {
            current = ii % 2;
            let next = (ii + 1) % 2;

            for jj in (0..chars2.len()).rev() {
                if chars1[ii] == chars2[jj] {
                    dp_staring_at[current][jj] = dp_staring_at[next][jj + 1] + 1;
                } else {
                    dp_staring_at[current][jj] =
                        dp_staring_at[current][jj + 1].max(dp_staring_at[next][jj]);
                }
            }
        }
        // current is the last used row index
        (total - 2 * dp_staring_at[current][0]) as i32
    }
    fn multiple(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<_>>();
        let mut result = i32::MAX;
        for ii in 0..chars.len() {
            result = result.min(Self::longest_common_subsequence(
                &chars[0..ii],
                &chars[ii..chars.len()],
            ));
            result = result.min(Self::longest_common_subsequence(
                &chars[0..ii],
                &chars[ii + 1..chars.len()],
            ));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_insertions("zzazz".to_string()), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_insertions("mbadm".to_string()), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::min_insertions("leetcode".to_string()), 5);
    }
}
