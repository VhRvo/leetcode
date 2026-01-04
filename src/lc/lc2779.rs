struct Solution;

impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut result = 0;
        let mut hi = 0;
        for (ii, num) in nums.iter().enumerate() {
            let target = *num + 2 * k;
            while hi < nums.len() && target >= nums[hi] {
                hi += 1;
            }
            result = result.max(hi - ii);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::maximum_beauty(vec![4, 6, 1, 2], 2), 3);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::maximum_beauty(vec![1, 1, 1, 1], 10), 4);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::maximum_beauty(vec![49, 26], 12), 2);
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::maximum_beauty(vec![13, 46, 71], 29), 3);
    }
}
