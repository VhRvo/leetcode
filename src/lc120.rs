struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = triangle.last().unwrap().clone();
        for row in triangle.iter().rev().skip(1) {
            for (jj, element) in row.iter().enumerate() {
                dp[jj] = dp[jj].min(dp[jj + 1]) + *element;
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        )
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10)
    }
}
