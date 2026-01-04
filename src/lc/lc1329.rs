struct Solution;

impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = mat.len();
        let cols = mat.first().unwrap().len();

        for no_col in 0..cols {
            let start = (0, no_col);
            sort(&mut mat, (0, no_col), length(rows, cols, start));
        }
        for no_row in 1..rows {
            let start = (no_row, 0);
            sort(&mut mat, (no_row, 0), length(rows, cols, start));
        }
        mat
    }
}

fn sort(mat: &mut [Vec<i32>], (start_row, start_col): (usize, usize), length: usize) {
    let mut vec = (0..length)
        .map(|offset| mat[start_row + offset][start_col + offset])
        .collect::<Vec<_>>();
    vec.sort();
    for (offset, value) in vec.iter().enumerate() {
        mat[start_row + offset][start_col + offset] = *value;
    }
}

fn length(rows: usize, cols: usize, (start_row, start_col): (usize, usize)) -> usize {
    (rows - start_row).min(cols - start_col)
}

#[test]
fn test() {
    assert_eq!(
        Solution::diagonal_sort(vec![
            vec![11, 25, 66, 1, 69, 7],
            vec![23, 55, 17, 45, 15, 52],
            vec![75, 31, 36, 44, 58, 8],
            vec![22, 27, 33, 25, 68, 4],
            vec![84, 28, 14, 11, 5, 50]
        ]),
        [
            [5, 17, 4, 1, 52, 7],
            [11, 11, 25, 45, 8, 69],
            [14, 23, 25, 44, 58, 15],
            [22, 27, 31, 36, 50, 66],
            [84, 28, 75, 33, 55, 68]
        ]
    );
}
