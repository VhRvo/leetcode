struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; 2];
        dp[0][1] = 1;
        for ii in 1..m + 1 {
            let previous = (ii - 1) % 2;
            let current = ii % 2;
            for jj in 1..n + 1 {
                dp[current][jj] = dp[previous][jj] + dp[current][jj - 1];
            }
        }
        dp[m % 2][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }
}
