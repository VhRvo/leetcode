struct Solution;
impl Solution {
    #[inline]
    fn low_bit(number: i32) -> i32 {
        number & -number
    }
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor_all = nums.iter().fold(0, |result, &elem| result ^ elem);
        let low_bit = Self::low_bit(xor_all);
        let mut result1 = 0;
        let mut result2 = 0;
        for number in nums {
            if number & low_bit == 0 {
                result1 ^= number;
            } else {
                result2 ^= number
            }
        }
        vec![result1, result2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut result = Solution::single_number(vec![1, 2, 1, 3, 2, 5]);
        result.sort();
        assert_eq!(result, [3, 5]);
    }
    #[test]
    fn test2() {
        let mut result = Solution::single_number(vec![-1, 0]);
        result.sort();
        assert_eq!(result, [-1, 0]);
    }
    #[test]
    fn test3() {
        let mut result = Solution::single_number(vec![0, 1]);
        result.sort();
        assert_eq!(result, [0, 1]);
    }
}
