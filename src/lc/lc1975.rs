struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut minimal_natural = i32::MAX;
        let mut maximal_negative = i32::MIN;
        let mut negative_count = 0;
        let mut result = 0 as i64;
        for row in matrix.iter() {
            for &element in row {
                if element >= 0 {
                    minimal_natural = minimal_natural.min(element);
                    result += element as i64;
                } else {
                    negative_count += 1;
                    maximal_negative = maximal_negative.max(element);
                    result -= element as i64;
                }
            }
        }
        if negative_count % 2 == 1 {
            result -= 2 * ((-maximal_negative).min(minimal_natural) as i64);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]),
            16
        );
    }
}
