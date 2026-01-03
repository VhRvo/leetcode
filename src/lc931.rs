struct Solution;

use std::i32;
impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let rows = matrix.len();
        let columns = matrix[0].len();
        let mut dp = vec![vec![0; columns + 2]; 2];
        for (ii, element) in matrix[0].iter().enumerate() {
            dp[0][ii + 1] = *element;
        }
        dp[0][0] = i32::MAX / 2;
        dp[0][columns + 1] = i32::MAX / 2;
        dp[1][0] = i32::MAX / 2;
        dp[1][columns + 1] = i32::MAX / 2;
        for (ii, row) in matrix.iter().enumerate().skip(1) {
            let previous = (ii - 1) % 2;
            let current = ii % 2;
            for (jj, element) in (1..).zip(row.iter()) {
                dp[current][jj] = dp[previous][jj - 1]
                    .min(dp[previous][jj])
                    .min(dp[previous][jj + 1])
                    + *element;
            }
        }
        dp[(rows - 1) % 2]
            .iter()
            .skip(1)
            .take(columns)
            .min()
            .unwrap()
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_falling_path_sum(
                [[2, 1, 3], [6, 5, 4], [7, 8, 9]]
                    .map(|array| array.to_vec())
                    .to_vec()
            ),
            13
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_falling_path_sum(
                [[-19, 57], [-40, -5]].map(|array| array.to_vec()).to_vec()
            ),
            -59
        )
    }
}
