struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let num_counts = {
            let mut map = BTreeMap::new();
            for num in nums {
                *map.entry(num).or_insert(0) += 1;
            }
            let mut num_counts: Vec<(i32, i32)> = Vec::with_capacity(map.len());
            for (num, total) in map {
                num_counts.push((num, total));
            }
            num_counts
        };
        let mut dp = vec![0; num_counts.len() + 1];
        dp[num_counts.len()] = 0;
        for i in (0..num_counts.len()).rev() {
            dp[i] = dp[i + 1];
            for j in i + 1..=num_counts.len() {
                if j == num_counts.len() {
                    dp[i] = dp[i].max(num_counts[i].0 * num_counts[i].1);
                    break;
                }
                if num_counts[j].0 != num_counts[i].0 + 1 {
                    dp[i] = dp[i].max(num_counts[i].0 * num_counts[i].1 + dp[j]);
                    break;
                }
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }
}
