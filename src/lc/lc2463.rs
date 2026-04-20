struct Solution;

// dp[ii][jj][kk]: 将 robot[ii..] 分配到 factory[jj..] 且 factory[jj] 已占用 kk 个名额时的最小总距离
mod plain {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort();
        factory.sort();
        let n = robot.len();
        let m = factory.len();
        let l = n;
        let mut dp = vec![vec![vec![i64::MAX / 2; l + 1]; m + 1]; n + 1];
        for jj in 0..m + 1 {
            for kk in 0..l + 1 {
                dp[n][jj][kk] = 0;
            }
        }
        for ii in (0..n).rev() {
            for jj in (0..m).rev() {
                let limit = factory[jj][1] as usize;
                for kk in 0..limit {
                    dp[ii][jj][kk] = (dp[ii + 1][jj][kk + 1]
                        + (robot[ii] - factory[jj][0]).abs() as i64)
                        .min(dp[ii][jj + 1][0]);
                }
                dp[ii][jj][limit] = dp[ii][jj + 1][0];
            }
        }
        dp[0][0][0]
    }
}

// 滚动数组优化: ii 维只依赖 ii+1, 压成 2 层
// jj 维保留完整长度, 因为转移同时需要 dp[next][jj] 和 dp[current][jj+1]
mod rolling {
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort();
        factory.sort();
        let n = robot.len();
        let m = factory.len();
        let l = n;
        let mut dp = vec![vec![vec![i64::MAX / 2; l + 1]; m + 1]; 2];
        for jj in 0..m + 1 {
            for kk in 0..l + 1 {
                dp[n % 2][jj][kk] = 0;
            }
        }
        for ii in (0..n).rev() {
            let current = ii % 2;
            let next = (ii + 1) % 2;
            for jj in 0..m + 1 {
                dp[current][jj][0] = i64::MAX / 2;
            }
            for jj in (0..m).rev() {
                let limit = factory[jj][1] as usize;
                for kk in 0..limit {
                    dp[current][jj][kk] = (dp[next][jj][kk + 1]
                        + (robot[ii] - factory[jj][0]).abs() as i64)
                        .min(dp[current][jj + 1][0]);
                }
                dp[current][jj][limit] = dp[current][jj + 1][0];
            }
        }
        dp[0][0][0]
    }
}

// ❌ 错误尝试: 将 jj 维也按奇偶压成 2 层
// 不可行原因: dp[next][current_jj][kk+1] 需要的是 "同一个工厂 jj" 在上一行的值,
// 但 jj%2 相同的不同工厂会写入同一个槽位, 导致状态互相覆盖 (aliasing)
// 例如 factory[0] 和 factory[2] 都映射到 slot 0, 后者会覆盖前者
mod rolling2 {
    #[allow(unused_imports)]
    use std::i64;

    #[allow(dead_code)]
    pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
        robot.sort();
        factory.sort();
        let n = robot.len();
        let m = factory.len();
        let l = n;
        let mut dp = vec![vec![vec![i64::MAX / 2; l + 1]; 2]; 2];
        for jj in 0..2 {
            for kk in 0..l + 1 {
                dp[n % 2][jj][kk] = 0;
            }
        }
        for ii in (0..n).rev() {
            let current = ii % 2;
            let next = (ii + 1) % 2;
            for jj in 0..2 {
                dp[current][jj][0] = i64::MAX / 2;
            }
            for jj in (0..m).rev() {
                let current_jj = jj % 2;
                let next_jj = (jj + 1) % 2;
                dp[current][current_jj][0] = i64::MAX / 2;

                let limit = factory[jj][1] as usize;
                for kk in 0..limit {
                    dp[current][current_jj][kk] = (dp[next][current_jj][kk + 1]
                        + (robot[ii] - factory[jj][0]).abs() as i64)
                        .min(dp[current][next_jj][0]);
                }
                dp[current][current_jj][limit] = dp[current][next_jj][0];
            }
        }
        dp[0][0][0]
    }
}

impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        // plain::minimum_total_distance(robot, factory)
        rolling::minimum_total_distance(robot, factory)
        // rolling2::minimum_total_distance(robot, factory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::minimum_total_distance(vec![0, 4, 6], vec![vec![2, 2], vec![6, 2]]),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::minimum_total_distance(vec![1, -1], vec![vec![-2, 1], vec![2, 1]]),
            2
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::minimum_total_distance(
                vec![9, 11, 99, 101],
                vec![
                    vec![10, 1],
                    vec![7, 1],
                    vec![14, 1],
                    vec![100, 1],
                    vec![96, 1],
                    vec![103, 1]
                ]
            ),
            6
        );
    }
}
