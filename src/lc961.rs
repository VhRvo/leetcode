struct Solution;

impl Solution {
    pub fn repeated_n_times(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len() / 2;
        let (a, b, c, d) = (nums[n - 2], nums[n - 1], nums[n], nums[n + 1]);
        if b == c {
            return b;
        } else if a == b {
            return a;
        } else if c == d {
            return c;
        }
        unreachable!()
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
