struct Solution;

impl Solution {
    fn all_one(
        row_prefix_sum: &[Vec<usize>],
        column_prefix_sum: &[Vec<usize>],
        row: usize,
        column: usize,
        length: usize,
    ) -> bool {
        (row_prefix_sum[row][column + length] - row_prefix_sum[row][column]) == length
            && (column_prefix_sum[row + length][column] - column_prefix_sum[row][column]) == length
    }
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let columns = matrix[0].len();
        let max_length = rows.min(columns);
        let mut dp = vec![vec![[false; 2]; columns]; rows];
        let mut result = 0;
        let mut row_prefix_sum = vec![vec![0; columns + 1]; rows];
        let mut column_prefix_sum = vec![vec![0; columns]; rows + 1];
        for (ii, row) in matrix.iter().enumerate() {
            for (jj, element) in row.iter().enumerate() {
                if *element == '1' {
                    dp[ii][jj][1] = true;
                    row_prefix_sum[ii][jj + 1] = row_prefix_sum[ii][jj] + 1;
                    column_prefix_sum[ii + 1][jj] = column_prefix_sum[ii][jj] + 1;
                    result = 1;
                } else {
                    row_prefix_sum[ii][jj + 1] = row_prefix_sum[ii][jj];
                    column_prefix_sum[ii + 1][jj] = column_prefix_sum[ii][jj];
                }
            }
        }
        for length in 2..=max_length {
            for (ii, row) in matrix.iter().enumerate().take(rows - length + 1) {
                for (jj, _) in row.iter().enumerate().take(columns - length + 1) {
                    let prev = (length + 1) % 2;
                    let current = length % 2;
                    dp[ii][jj][current] = false;
                    if dp[ii + 1][jj + 1][prev] && Self::all_one(&row_prefix_sum, &column_prefix_sum, ii, jj, length) {
                        dp[ii][jj][current] = true;
                        result = result.max(length);
                    }
                }
            }
        }
        (result * result) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximal_square(
                [
                    ["1", "0", "1", "0", "0"],
                    ["1", "0", "1", "1", "1"],
                    ["1", "1", "1", "1", "1"],
                    ["1", "0", "0", "1", "0"]
                ]
                .map(|row| row.map(|c| c.chars().next().unwrap()).to_vec())
                .to_vec()
            ),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximal_square(
                [["0", "1"], ["1", "0"]]
                    .map(|row| row.map(|c| c.chars().next().unwrap()).to_vec())
                    .to_vec()
            ),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::maximal_square(
                [["0"]]
                    .map(|row| row.map(|c| c.chars().next().unwrap()).to_vec())
                    .to_vec()
            ),
            0
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::maximal_square(
                [["1", "1"], ["1", "1"]]
                    .map(|row| row.map(|c| c.chars().next().unwrap()).to_vec())
                    .to_vec()
            ),
            4
        );
    }
}
