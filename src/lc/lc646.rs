struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut pairs = pairs
            .into_iter()
            .map(|pair| (pair[0], pair[1]))
            .collect::<Vec<_>>();
        pairs.sort_by(|left, right| match left.0.cmp(&right.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => left.1.cmp(&right.0),
            Ordering::Greater => Ordering::Greater,
        });
        let mut dp = vec![0; pairs.len() + 1];
        let mut result = 0;
        for ii in (0..pairs.len()).rev() {
            let mut last_index = pairs.len();
            for jj in (ii + 1..pairs.len()).rev() {
                if pairs[ii].1 < pairs[jj].0 {
                    if dp[jj] > dp[last_index] {
                        last_index = jj;
                    }
                }
            }
            dp[ii] = 1 + dp[last_index];
            result = result.max(dp[ii]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
            2
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_longest_chain(vec![vec![1, 2], vec![7, 8], vec![4, 5]]),
            3
        )
    }
}
