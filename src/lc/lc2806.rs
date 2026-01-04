struct Solution;

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        100 - (purchase_amount + 5) / 10 * 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::account_balance_after_purchase(9), 90);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::account_balance_after_purchase(15), 80);
    }
}
