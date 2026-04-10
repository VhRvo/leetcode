struct Solution;

use std::{collections::HashMap, i32};
impl Solution {
    fn get_distance(ii: usize, kk: usize) -> i32 {
        return (kk - ii) as i32;
    }
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let map = {
            let mut map = HashMap::new();
            for (ii, num) in nums.into_iter().enumerate() {
                map.entry(num).or_insert(Vec::with_capacity(3)).push(ii);
            }
            map
        };
        let result = {
            let mut result = i32::MAX;
            for indices in map.values() {
                let mut ii = 2;
                while ii < indices.len() {
                    result = result.min(Solution::get_distance(indices[ii - 2], indices[ii]));
                    ii += 1;
                }
            }
            result
        };
        if result == i32::MAX {
            -1
        } else {
            2 * result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::minimum_distance(vec![1, 2, 1, 1, 3]), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]), 8);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::minimum_distance(vec![1]), -1);
    }
}
