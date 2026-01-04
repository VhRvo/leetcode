struct Solution;

// rolling array
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let n = n as usize;
        const K: usize = 3;
        const ROUND: usize = K + 1;
        let mut dp = [0; ROUND];
        dp[0] = 0;
        dp[1] = 1;
        dp[2] = 1;
        for i in K..=n {
            let current: usize = i % ROUND;
            let mut sum = 0;
            for k in 1..=K {
                sum += dp[(i + ROUND - k) % ROUND];
            }
            dp[current] = sum;
        }
        dp[n % ROUND]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::tribonacci(4), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
