struct Solution;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let length = grid.len();
        let mut row_maximum = vec![0; length];
        let mut column_maximum = vec![0; length];
        for (row, line) in grid.iter().enumerate() {
            for (column, cell) in line.iter().copied().enumerate() {
                row_maximum[row] = row_maximum[row].max(cell);
                column_maximum[column] = column_maximum[column].max(cell);
            }
        }
        grid.iter()
            .enumerate()
            .map(|(row, line)| {
                line.iter()
                    .copied()
                    .enumerate()
                    .map(|(column, cell)| row_maximum[row].min(column_maximum[column]) - cell)
                    .sum::<i32>()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_increase_keeping_skyline(
                [[3, 0, 8, 4], [2, 4, 5, 7], [9, 2, 6, 3], [0, 3, 1, 0]]
                    .map(|array| array.to_vec())
                    .to_vec()
            ),
            35
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::max_increase_keeping_skyline(
                [[0, 0, 0], [0, 0, 0], [0, 0, 0]]
                    .map(|array| array.to_vec())
                    .to_vec()
            ),
            0
        );
    }
}
