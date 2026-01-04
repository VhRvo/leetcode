struct Solution;

impl Solution {
    pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let columns = matrix.first().unwrap().len();
        let mut max_in_column = vec![-1; columns];
        for row in matrix.iter() {
            for (max, cell) in max_in_column.iter_mut().zip(row) {
                *max = *cell.max(max);
            }
        }

        for row in matrix.iter_mut() {
            for (max, cell) in max_in_column.iter().zip(row) {
                if *cell == -1 {
                    *cell = *max;
                }
            }
        }

        matrix
    }
}
