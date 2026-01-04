struct Solution;

// rolling array
impl Solution {
    pub fn fib(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = [0; 3];
        dp[0] = 0;
        dp[1] = 1;
        // if you use 1 instead of 2 as initial value,
        // you introduce a subtle bug.
        /* for i in 1..=n { */
        for i in 2..=n {
            let prev2 = (i + 2) % 3;
            let prev1 = (i + 1) % 3;
            let current = i % 3;
            dp[current] = dp[prev2] + dp[prev1];
        }
        dp[n % 3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::fib(2), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::fib(3), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::fib(4), 3);
    }
}
