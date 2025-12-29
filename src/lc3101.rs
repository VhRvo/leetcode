struct Solution;

impl Solution {
    #[inline]
    fn sum_inclusive(lo: usize, hi: usize) -> usize {
        (lo + hi) * (hi - lo + 1) / 2
    }
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut lo = 0;
        while lo < nums.len() {
            let mut hi = lo + 1;
            while hi < nums.len() && (hi == nums.len() || nums[hi - 1] != nums[hi]) {
                hi += 1;
            }
            result += Self::sum_inclusive(1, hi - lo);
            lo = hi;
        }
        result as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_alternating_subarrays(vec![0, 1, 1, 1]), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_alternating_subarrays(vec![1, 0, 1, 0]), 10);
    }
}
