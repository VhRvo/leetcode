struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1 = s1.bytes().collect::<Vec<_>>();
        let s2 = s2.bytes().collect::<Vec<_>>();
        let mut dp = vec![vec![0; s2.len() + 1]; 2];
        {
            // last_row is povit row
            let last_row = s1.len() % 2;
            let mut last_row_sum = 0;
            // reverse loop
            for (jj, &element) in s2.iter().enumerate().rev() {
                last_row_sum += element as i32;
                dp[last_row][jj] = last_row_sum;
            }
        }

        // last_column is povit column
        let last_column = s2.len();
        let mut last_column_sum = 0;
        for ii in (0..s1.len()).rev() {
            let current = ii % 2;
            let next = (ii + 1) % 2;

            last_column_sum += s1[ii] as i32;
            dp[current][last_column] = last_column_sum;
            for jj in (0..s2.len()).rev() {
                if s1[ii] == s2[jj] {
                    dp[current][jj] = dp[next][jj + 1];
                } else {
                    dp[current][jj] = (s1[ii] as i32 + dp[next][jj]).min(s2[jj] as i32 + dp[current][jj + 1]);
                }

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
        println!("{} {}", b's', b't');
        assert_eq!(
            Solution::minimum_delete_sum("sea".to_string(), "eat".to_string()),
            231
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::minimum_delete_sum("delete".to_string(), "leet".to_string()),
            403
        );
    }
}
