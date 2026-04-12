struct Solution;

mod plain {
    pub fn minimum_distance(word: String) -> i32 {
        let word: Vec<usize> = word.chars().map(|ch| (ch as u8 - b'A') as usize).collect();
        let coordinates: Vec<(i32, i32)> =
            (0..26).into_iter().map(|idx| (idx / 6, idx % 6)).collect();
        let get_distance = |origin: usize, destination: usize| -> i32 {
            if origin == 0 {
                0
            } else {
                let (origin_x, origin_y) = coordinates[origin - 1];
                let (dest_x, dest_y) = coordinates[destination];
                (origin_x - dest_x).abs() + (origin_y - dest_y).abs()
            }
        };
        let length = word.len();
        const WIDTH: usize = 26;
        let mut dp = vec![vec![vec![i32::MAX / 2; WIDTH + 1]; WIDTH + 1]; length + 1];
        for ii in 0..WIDTH + 1 {
            for jj in 0..WIDTH + 1 {
                dp[length][ii][jj] = 0;
            }
        }
        for ii in (0..length).rev() {
            for jj in 0..WIDTH + 1 {
                for kk in 0..WIDTH + 1 {
                    let choice1_value = get_distance(jj, word[ii]);
                    let choice2_value = get_distance(kk, word[ii]);
                    dp[ii][jj][kk] = (dp[ii + 1][word[ii] + 1][kk] + choice1_value)
                        .min(dp[ii + 1][jj][word[ii] + 1] + choice2_value);
                }
            }
        }
        dp[0][0][0]
    }
}

mod rolling {
    pub fn minimum_distance(word: String) -> i32 {
        let word: Vec<usize> = word.chars().map(|ch| (ch as u8 - b'A') as usize).collect();
        let coordinates: Vec<(i32, i32)> =
            (0..26).into_iter().map(|idx| (idx / 6, idx % 6)).collect();
        let get_distance = |origin: usize, destination: usize| -> i32 {
            if origin == 0 {
                0
            } else {
                let (origin_x, origin_y) = coordinates[origin - 1];
                let (dest_x, dest_y) = coordinates[destination];
                (origin_x - dest_x).abs() + (origin_y - dest_y).abs()
            }
        };
        let length = word.len();
        const WIDTH: usize = 26;
        let mut dp = vec![vec![vec![i32::MAX / 2; WIDTH + 1]; WIDTH + 1]; 2];
        for ii in 0..WIDTH + 1 {
            for jj in 0..WIDTH + 1 {
                dp[length % 2][ii][jj] = 0;
            }
        }
        for ii in (0..length).rev() {
            let current = ii % 2;
            let next = (ii + 1) % 2;
            for jj in 0..WIDTH + 1 {
                for kk in 0..WIDTH + 1 {
                    let choice1_value = get_distance(jj, word[ii]);
                    let choice2_value = get_distance(kk, word[ii]);
                    dp[current][jj][kk] = (dp[next][word[ii] + 1][kk] + choice1_value)
                        .min(dp[next][jj][word[ii] + 1] + choice2_value);
                }
            }
        }
        dp[0][0][0]
    }
}

mod symmetry {
    pub fn minimum_distance(word: String) -> i32 {
        let word: Vec<usize> = word.chars().map(|ch| (ch as u8 - b'A') as usize).collect();
        let coordinates: Vec<(i32, i32)> =
            (0..26).into_iter().map(|idx| (idx / 6, idx % 6)).collect();
        let get_distance = |origin: usize, destination: usize| -> i32 {
            let (origin_x, origin_y) = coordinates[origin];
            let (dest_x, dest_y) = coordinates[destination];
            (origin_x - dest_x).abs() + (origin_y - dest_y).abs()
        };
        let length = word.len();
        const WIDTH: usize = 26;
        // symmetry 版本的 dp[ii][ch] 代表主手指在 ii 位置, 副手指在 ch 字符上,
        // 和 non-symmetry 版本的区别是, ii 位置已经被主手指按下了
        let mut dp = vec![vec![i32::MAX / 2; WIDTH + 1]; length];
        for ch in 0..WIDTH + 1 {
            dp[length - 1][ch] = 0;
        }
        for ii in (0..length - 1).rev() {
            for jj in 0..WIDTH + 1 {
                let choice1_value = if jj == 0 {
                    0
                } else {
                    get_distance(jj - 1, word[ii + 1])
                };
                let choice2_value = get_distance(word[ii], word[ii + 1]);
                dp[ii][jj] =
                    (dp[ii + 1][word[ii] + 1] + choice1_value).min(dp[ii + 1][jj] + choice2_value);
            }
        }
        dp[0][0]
    }
}

mod symmetry_rolling {
    pub fn minimum_distance(word: String) -> i32 {
        let word: Vec<usize> = word.chars().map(|ch| (ch as u8 - b'A') as usize).collect();
        let coordinates: Vec<(i32, i32)> =
            (0..26).into_iter().map(|idx| (idx / 6, idx % 6)).collect();
        let get_distance = |origin: usize, destination: usize| -> i32 {
            let (origin_x, origin_y) = coordinates[origin];
            let (dest_x, dest_y) = coordinates[destination];
            (origin_x - dest_x).abs() + (origin_y - dest_y).abs()
        };
        let length = word.len();
        const WIDTH: usize = 26;
        let mut dp = vec![vec![i32::MAX / 2; WIDTH + 1]; 2];
        for ch in 0..WIDTH + 1 {
            dp[(length - 1) % 2][ch] = 0;
        }
        for ii in (0..length - 1).rev() {
            let current = ii % 2;
            let next = (ii + 1) % 2;
            for jj in 0..WIDTH + 1 {
                let choice1_value = if jj == 0 {
                    0
                } else {
                    get_distance(jj - 1, word[ii + 1])
                };
                let choice2_value = get_distance(word[ii], word[ii + 1]);
                dp[current][jj] =
                    (dp[next][word[ii] + 1] + choice1_value).min(dp[next][jj] + choice2_value);
            }
        }
        dp[0][0]
    }
}

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        // plain::minimum_distance(word)
        // rolling::minimum_distance(word)
        // symmetry::minimum_distance(word)
        symmetry_rolling::minimum_distance(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_distance("CAKE".to_string()), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_distance("HAPPY".to_string()), 6);
    }
}
