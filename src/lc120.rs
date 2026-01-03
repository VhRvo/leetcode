struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let rows = triangle.len();
        let columns = triangle.last().unwrap().len();
        let mut dp = vec![vec![0; columns]; 2];
        dp[(rows - 1) % 2] = triangle.last().unwrap().clone();
        for (ii, row) in triangle.iter().enumerate().rev().skip(1) {
            let current = ii % 2;
            let next = (ii + 1) % 2;
            for (jj, element) in row.iter().enumerate() {
                dp[current][jj] = dp[next][jj].min(dp[next][jj + 1]) + *element;
            }
        }
        dp[0][0]
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
