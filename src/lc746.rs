struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = [0; 3];
        const ROUND: usize = 3;
        dp[0] = 0;
        dp[1] = 0;
        for i in 2..=cost.len() {
            let prev2 = i - 2;
            let prev1 = i - 1;
            let round_prev2 = (prev2 + ROUND) % ROUND;
            let round_prev1 = (prev1 + ROUND) % ROUND;
            let current = i % ROUND;
            // Note: dp is indexed by round-robin (i % ROUND), while cost is indexed by actual position (i-1, i-2)
            dp[current] = (dp[round_prev2] + cost[prev2]).min(dp[round_prev1] + cost[prev1]);
        }
        dp[cost.len() % ROUND]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }

    #[test]
    fn test() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![0, 0, 1, 1]), 1);
    }
}
