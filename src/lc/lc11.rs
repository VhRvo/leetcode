struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut hi = height.len() - 1;
        #[inline]
        fn calc_area(height: &[i32], lo: usize, hi: usize) -> i32 {
            (hi - lo) as i32 * height[lo].min(height[hi])
        }
        let mut result = 0;
        while lo < hi {
            result = result.max(calc_area(&height, lo, hi));
            println!("result: {lo}, {hi}, {result}");
            if height[lo] > height[hi] {
                hi -= 1;
            } else {
                lo += 1;
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
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::max_area(vec![1, 3, 2, 5, 25, 24, 5]), 24);
    }
}
