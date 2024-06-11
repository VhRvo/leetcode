struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let height = matrix.len();
        let width = matrix[0].len();
        (0..width)
            .map(|ii| (0..height).map(|jj| matrix[jj][ii]).collect())
            .collect()
    }
}
