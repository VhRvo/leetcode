struct Solution;

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut result = i32::MAX;
        for (ii, num) in nums.into_iter().enumerate() {
            if num == target {
                result = result.min((ii as i32 - start).abs());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_min_distance(vec![1, 2, 3, 4, 5], 5, 3), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::get_min_distance(vec![1], 1, 0), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::get_min_distance(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0),
            0
        );
    }
}
