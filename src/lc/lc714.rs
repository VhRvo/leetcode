struct Solution;

// rolling array
use std::i32;
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut hold = i32::MIN / 2;
        let mut not_hold = 0;
        for i in (0..prices.len()).rev() {
            hold = hold.max(not_hold + prices[i]);
            not_hold = not_hold.max(hold - prices[i] - fee);
        }
        not_hold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6)
    }
}
