struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;

        let mut ii = 0;
        let mut jj = numbers.len() - 1;
        while ii < jj {
            if ii > 0 && numbers[ii - 1] == numbers[ii] {
                ii += 1;
                continue;
            }
            if jj < numbers.len() - 1 && numbers[jj] == numbers[jj + 1] {
                jj -= 1;
                continue;
            }
            match (numbers[ii] + numbers[jj]).cmp(&target) {
                Ordering::Less => {
                    ii += 1;
                    continue;
                }
                Ordering::Equal => {
                    return vec![ii as i32 + 1, jj as i32 + 1];
                }
                Ordering::Greater => {
                    jj -= 1;
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2])
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3])
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2])
    }
}
