struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut minimal_natural = i32::MAX;
        let mut negatives = Vec::with_capacity(matrix.len());
        let mut result = 0 as i64;
        for row in matrix.iter() {
            for &element in row {
                if element >= 0 {
                    minimal_natural = minimal_natural.min(element);
                    result += element as i64;
                } else {
                    negatives.push(-element);
                }
            }
        }
        negatives.sort();
        let rest = if negatives.len() % 2 == 0 {
            0
        } else {
            if minimal_natural < negatives[0] {
                result += (negatives[0] - 2 * minimal_natural) as i64;
            } else {
                result -= negatives[0] as i64;
            }
            1
        };
        for &negative in negatives.iter().skip(rest) {
            result += negative as i64;
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
