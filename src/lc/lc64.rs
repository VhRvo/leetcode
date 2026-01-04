use std::i32;

struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let row = grid.len();
        let column = grid[0].len();
        let mut dp = vec![vec![0; column + 1]; 2];
        dp[0][1] = 0;
        // 1, 1 -> i, 0 is impossible
        dp[1][0] = i32::MAX;
        for element in dp[0].iter_mut().skip(1) {
            // 1, 1 -> 0, j is impossible
            *element = i32::MAX;
        }
        for ii in 1..row + 1 {
            let current = ii % 2;
            let previous = (ii - 1) % 2;
            for jj in 1..column + 1 {
                dp[current][jj] = dp[previous][jj].min(dp[current][jj - 1]) + grid[ii - 1][jj - 1];
            }
        }
        dp[row % 2][column]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_path_sum(
                [[1, 3, 1], [1, 5, 1], [4, 2, 1]]
                    .map(|array| array.to_vec())
                    .to_vec()
            ),
            7
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_path_sum([[1, 2, 3], [4, 5, 6]].map(|array| array.to_vec()).to_vec()),
            12
        );
    }
}
