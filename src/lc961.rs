struct Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut window = [nums[0], nums[1]];
        if window[0] == window[1] {
            return window[0];
        }
        if window.contains(&nums[3]) {
            return nums[3];
        }
        for i in 2..nums.len() {
            if window.contains(&nums[i]) {
                return nums[i];
            }
            window[i % 2] = nums[i];
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::repeated_n_times(vec![1, 2, 3, 3],), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2],), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4],), 5);
    }
}
