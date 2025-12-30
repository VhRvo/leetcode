struct Solution;

impl Solution {
    fn all_one(matrix: &[Vec<char>], row: usize, column: usize, length: usize) -> bool {
        for ii in 0..length {
            if matrix[row + ii][column] != '1' {
                return false;
            }
            if matrix[row][column + ii] != '1' {
                return false
            }
        }
        return true;
    }
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let columns = matrix[0].len();
        let max_length = rows.min(columns);
        let mut dp = vec![vec![vec![false; max_length + 1]; columns]; rows];
        let mut result = 0;
        for (ii, row) in matrix.iter().enumerate() {
            for (jj, element) in row.iter().enumerate() {
                if *element == '1' {
                    dp[ii][jj][1] = true;
                    result = 1;
                }
            }
        }
        for length in 2..=max_length {
            for (ii, row) in matrix.iter().enumerate().take(rows - length + 1) {
                for (jj, _) in row.iter().enumerate().take(columns - length + 1) {
                    if dp[ii + 1][jj + 1][length - 1] && Self::all_one(&matrix, ii, jj, length) {
                        dp[ii][jj][length] = true;
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
                [["1","1"],["1","1"]]
                    .map(|row| row.map(|c| c.chars().next().unwrap()).to_vec())
                    .to_vec()
            ),
            4
        );
    }
}
