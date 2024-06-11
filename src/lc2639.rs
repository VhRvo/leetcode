struct Solution;
impl Solution {
    // pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
    //     let rows = grid.len();
    //     let columns = grid.first().unwrap().len();
    //     let mut result = vec![0; columns];
    //     for row in grid {
    //         for (result_cell, cell) in result.iter_mut().zip(row) {
    //             *result_cell = (*result_cell).max(length_of_num(cell));
    //         }
    //     }
    //     result
    // }
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        // let rows = grid.len();
        let columns = grid.first().unwrap().len();
        let mut min_vec = vec![i32::MAX; columns];
        let mut max_vec = vec![i32::MIN; columns];
        for row in grid {
            for ((min, max), cell) in min_vec.iter_mut().zip(max_vec.iter_mut()).zip(row) {
                *min = (*min).min(cell);
                *max = (*max).max(cell);
            }
        }
        min_vec
            .into_iter()
            .zip(max_vec)
            .map(|(min, max)| length_of_num(min).max(length_of_num(max)))
            .collect()
    }
}

fn length_of_num(num: i32) -> i32 {
    num.abs().to_string().len() as i32 + (num.is_negative() as i32)
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_column_width(vec![vec![-15, 1, 3], vec![15, 7, 12], vec![5, 6, -2]]),
        vec![3, 1, 2]
    )
}
