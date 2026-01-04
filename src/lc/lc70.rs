struct Solution;

// native
/*
use std::vec;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[n] = 1;
        dp[n - 1] = 1;
        for i in (0..n-1).rev() {
            dp[i] = dp[i + 1] + dp[i + 2];
        }
        dp[0]
    }
}
*/

// rolling array
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = [0; 3];
        dp[n % 3] = 1;
        dp[(n - 1) % 3] = 1;
        for i in (0..n - 1).rev() {
            let current = i % 3;
            let next1 = (i + 1) % 3;
            let next2 = (i + 2) % 3;
            dp[current] = dp[next1] + dp[next2];
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
