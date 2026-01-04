struct Solution;

impl Solution {
    // pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    //     let width = matrix.len();
    //     let half = width / 2;
    //     for lo in 0. .half {
    //         let hi = width - lo;
    //         for _ in (lo .. hi - 1) {
    //             rotate(lo, hi, matrix);
    //         }
    //     }
    // }
    pub fn rotate(matrix: &mut [Vec<i32>]) {
        use std::ops::Div;

        // matrix[i][j] -> matrix[j][n - i - 1]
        let width = matrix.len();
        let row_half = (width as f64).div(2.0).floor() as usize;
        let col_half = (width as f64).div(2.0).ceil() as usize;

        for ii in 0..row_half {
            for jj in 0..col_half {
                // (ii, jj) -> (jj, width - ii - 1)
                // (jj, width - ii - 1) -> (width - ii - 1, width - jj - 1)
                // (width - ii - 1, width - jj - 1) -> (width - jj - 1, ii)
                // (width - jj - 1, ii) -> (ii, jj)
                let temp = matrix[ii][jj];
                matrix[ii][jj] = matrix[width - jj - 1][ii];
                matrix[width - jj - 1][ii] = matrix[width - ii - 1][width - jj - 1];
                matrix[width - ii - 1][width - jj - 1] = matrix[jj][width - ii - 1];
                matrix[jj][width - ii - 1] = temp;
            }
        }
    }
    // pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    //     let width = matrix.len();
    //     for ii in (0..width) {
    //         for jj in (ii + 1..width) {
    //             let temp = matrix[ii][jj];
    //             matrix[ii][jj] = matrix[jj][ii];
    //             matrix[jj][ii] = temp;
    //             // mem::swap(&mut matrix[ii][jj], &mut matrix[jj][ii]);
    //         }
    //     }
    //     for line in matrix.iter_mut() {
    //         line.reverse();
    //     }
    // }
}

fn rotate(lo: usize, hi: usize, matrix: &mut [Vec<i32>]) {
    let left_up = matrix[lo][lo];
    for no_row in lo + 1..hi {
        matrix[no_row - 1][lo] = matrix[no_row][lo];
    }
    for no_col in lo + 1..hi {
        matrix[hi - 1].swap(no_col - 1, no_col);
        // matrix[hi - 1][no_col - 1] = matrix[hi - 1][no_col];
    }
    for no_row in (lo..hi - 1).rev() {
        matrix[no_row + 1][hi - 1] = matrix[no_row][hi - 1];
    }
    for no_col in (lo + 1..hi - 1).rev() {
        matrix[lo].swap(no_col + 1, no_col);
        // matrix[lo][no_col + 1] = matrix[lo][no_col];
    }
    matrix[lo][lo + 1] = left_up;
}
