struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let row = obstacle_grid.len();
        let column = obstacle_grid[0].len();
        let mut dp = vec![vec![0; column + 1]; 2];
        dp[0][1] = 1;
        for ii in 1..row + 1 {
            let previous = (ii - 1) % 2;
            let current = ii % 2;
            for jj in 1..column + 1 {
                if obstacle_grid[ii - 1][jj - 1] == 1 {
                    dp[current][jj] = 0;
                } else {
                    dp[current][jj] = dp[previous][jj] + dp[current][jj - 1];
                }
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
            Solution::unique_paths_with_obstacles(
                [[0, 0, 0], [0, 1, 0], [0, 0, 0]]
                    .map(|array| array.to_vec())
                    .to_vec()
            ),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(
                [[0, 1], [0, 0]].map(|array| array.to_vec()).to_vec()
            ),
            1
        );
    }
}
