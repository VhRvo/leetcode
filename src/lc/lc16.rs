struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;

        nums.sort();
        let mut result = i32::MAX;
        for ii in 0..nums.len() - 2 {
            if ii > 0 && nums[ii - 1] == nums[ii] {
                continue;
            }
            let mut jj = ii + 1;
            let mut kk = nums.len() - 1;
            while jj < kk {
                if jj > ii + 1 && nums[jj - 1] == nums[jj] {
                    jj += 1;
                    continue;
                }
                if kk < nums.len() - 1 && nums[kk] == nums[kk + 1] {
                    kk -= 1;
                    continue;
                }
                let sum = nums[ii] + nums[jj] + nums[kk];
                if sum.abs_diff(target) < result.abs_diff(target) {
                    result = sum;
                }
                match sum.cmp(&target) {
                    Ordering::Less => {
                        jj += 1;
                    }
                    Ordering::Equal => {
                        jj += 1;
                        kk -= 1;
                    }
                    Ordering::Greater => {
                        kk -= 1;
                    }
                }
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
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }
}
