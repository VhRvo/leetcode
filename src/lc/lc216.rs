struct Solution;

impl Solution {
    fn backtracing(
        current: i32,
        sequence: &mut Vec<i32>,
        sum: i32,
        k: usize,
        target: i32,
        result: &mut Vec<Vec<i32>>,
    ) {
        if sequence.len() == k {
            if sum == target {
                result.push(sequence.clone())
            }
        } else if sum >= target || current > 9 {
            return;
        } else {
            Self::backtracing(current + 1, sequence, sum, k, target, result);
            sequence.push(current);
            Self::backtracing(current + 1, sequence, sum + current, k, target, result);
            sequence.pop();
        }
    }

    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut sequence = vec![];
        Self::backtracing(1, &mut sequence, 0, k as usize, n, &mut result);
        result
    }
}
