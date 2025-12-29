struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;

        nums.sort();
        let target = 0;
        let mut result = Vec::new();
        // let last_two_sum: i32 = nums.last_chunk::<2>().unwrap().iter().sum();
        let last_two_sum = nums[nums.len() - 2] + nums[nums.len() - 1];
        for ii in 0..nums.len() - 2 {
            let ii_num = nums[ii];
            if ii > 0 && nums[ii - 1] == ii_num {
                continue;
            }
            if ii_num + nums[ii + 1] + nums[ii + 2] > target {
                break;
            }
            if ii_num + last_two_sum < target {
                continue;
            }
            let mut jj = ii + 1;
            let mut kk = nums.len() - 1;
            while jj < kk {
                let jj_num = nums[jj];
                let kk_num = nums[kk];
                if jj > ii + 1 && nums[jj - 1] == jj_num {
                    jj += 1;
                    continue;
                }
                if kk < nums.len() - 1 && kk_num == nums[kk + 1] {
                    kk -= 1;
                    continue;
                }
                match (ii_num + jj_num + kk_num).cmp(&target) {
                    Ordering::Less => {
                        jj += 1;
                    }
                    Ordering::Equal => {
                        result.push(vec![ii_num, jj_num, kk_num]);
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
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<_>>::new());
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
    #[test]
    fn test4() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
    }
    #[test]
    fn test5() {
        assert_eq!(
            Solution::three_sum(vec![-4, -2, -1, 5, 6]),
            vec![vec![-4, -2, 6], vec![-4, -1, 5]]
        );
    }
}
