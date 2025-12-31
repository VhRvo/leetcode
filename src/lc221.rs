struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let columns = matrix[0].len();
        let mut dp = vec![vec![0; columns + 1]; rows + 1];
        let mut result = 0;

        for (ii, row) in (1..).zip(matrix.iter()) {
            for (jj, &element) in (1..).zip(row.iter()) {
                if element == '0' {
                    continue;
                }
                dp[ii][jj] = dp[ii - 1][jj - 1].min(dp[ii - 1][jj]).min(dp[ii][jj - 1]) + 1;
                result = result.max(dp[ii][jj]);
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

    #[test]
    fn test5() {
        assert_eq!(
            Solution::maximal_square(
                [
                    ["1", "1", "1", "1", "0"],
                    ["1", "1", "1", "1", "0"],
                    ["1", "1", "1", "1", "1"],
                    ["1", "1", "1", "1", "1"],
                    ["0", "0", "1", "1", "1"]
                ]
                .map(|row| row.map(|c| c.chars().next().unwrap()).to_vec())
                .to_vec()
            ),
            16
        );
    }
}
