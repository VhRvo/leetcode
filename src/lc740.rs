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
        // num_counts[num_counts.len()].0 as a dummy value, which is not equal to any valid number + 1
        let mut dp = vec![0; num_counts.len() + 1];
        dp[num_counts.len()] = 0;
        for i in (0..num_counts.len()).rev() {
            dp[i] = dp[i + 1];

            // Find the next selectable position (first position where the number is not current number + 1)
            // let next_pos = {
            //     let mut j = i + 1;
            //     while j < num_counts.len() && num_counts[j].0 == num_counts[i].0 + 1 {
            //         j += 1;
            //     }
            //     j
            // };

            // Or use simple loop
            // let mut next_pos = num_counts.len();
            // for j in i + 1..num_counts.len() {
            //     if num_counts[j].0 != num_counts[i].0 + 1 {
            //         next_pos = j;
            //         break;
            //     }
            // }

            // Or use iterator
            // let next_pos = (i + 1..num_counts.len())
            //     .find(|&j| num_counts[j].0 != num_counts[i].0 + 1)
            //     .unwrap_or(num_counts.len());

            // Or use binary search
            let next_pos = num_counts[i + 1..num_counts.len()]
                .partition_point(|(num, _)| num_counts[i].0 + 1 >= *num) + i + 1;

            let earn = num_counts[i].0 * num_counts[i].1;
            dp[i] = dp[i].max(earn + dp[next_pos]);
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

    #[test]
    fn test3() {
        assert_eq!(
            Solution::delete_and_earn(vec![1, 1, 1, 2, 4, 5, 5, 5, 6]),
            18
        );
    }
}
