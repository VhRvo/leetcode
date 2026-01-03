struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        const MODULE: i64 = 1e9 as i64 + 7;
        let n = n as usize;
        let encoding_map = {
            let mut encoding_map = HashMap::new();
            let mut encoding = 0;
            for ii in 0..3 {
                for jj in 0..3 {
                    for kk in 0..3 {
                        if ii != jj && jj != kk {
                            encoding_map.insert(encoding, (ii, jj, kk));
                            encoding += 1;
                        }
                    }
                }
            }
            encoding_map
        };
        let edge_map = {
            let mut edge_map = vec![Vec::new(); 12];
            for (encoding, (first, second, third)) in encoding_map.iter() {
                let mut inner_encoding = 0;
                for ii in 0..3 {
                    for jj in 0..3 {
                        for kk in 0..3 {
                            if ii != jj && jj != kk {
                                if *first != ii && *second != jj && *third != kk {
                                    edge_map[*encoding].push(inner_encoding);
                                }
                                inner_encoding += 1;
                            }
                        }
                    }
                }
            }
            edge_map
        };
        let mut dp = vec![vec![0; 12]; n + 1];
        for encoding in 0..12 {
            dp[1][encoding] = 1;
        }
        for ii in 2..n + 1 {
            for encoding in 0..12 {
                for &last_encoding in edge_map[encoding].iter() {
                    dp[ii][encoding] = (dp[ii][encoding] + dp[ii - 1][last_encoding]) % MODULE;
                }
            }
        }
        let mut result = 0;
        for encoding in 0..12 {
            result = (result + dp[n][encoding]) % MODULE;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_of_ways(1), 12);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::num_of_ways(5000), 30228214);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::num_of_ways(2), 54);
    }
}
