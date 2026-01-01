struct Solution;

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let row = row as usize;
        let col = col as usize;
        let mut connection = {
            let mut connection = vec![vec![0; col + 2]; row + 2];
            connection[1][1] = 2;
            connection[1][col] = 2;
            connection[row][1] = 2;
            connection[row][col] = 2;
            for ii in 2..row {
                connection[ii][1] = 3;
                connection[ii][col] = 3;
            }
            for jj in 2..col {
                connection[1][jj] = 3;
                connection[row][jj] = 3;
            }
            for ii in 2..row {
                for jj in 2..col {
                    connection[ii][jj] = 4;
                }
            }
            connection
        };
        println!("{:?}", connection);
        let mut row_count = vec![0; row + 2];
        let mut result = 0;
        'outer:
        for cell in cells.iter() {
            let x = cell[0] as usize;
            let y = cell[1] as usize;
            if connection[x][y] > 1 {
                row_count[x] += 1;
                if row_count[x] == col {
                    break;
                }
            }
            connection[x][y] = 0;
            for (x_offset, y_offset) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_x = (x as isize + x_offset) as usize;
                let new_y = (y as isize + y_offset) as usize;
                if new_x < 1 || new_x > row || new_y < 1 || new_y > col {
                    continue;
                }
                if connection[new_x][new_y] == 0 {
                    continue;
                }
                connection[new_x][new_y] -= 1;
                if connection[new_x][new_y] == 0 {
                    continue;
                }
                if connection[new_x][new_y] > 1 {
                    continue;
                }
                row_count[new_x] += 1;
                if row_count[new_x] == col {
                    break 'outer;
                }
            }
            result += 1;
        }
        println!("{:?}", connection);
        result

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::latest_day_to_cross(
                2,
                2,
                [[1, 1], [2, 1], [1, 2], [2, 2]]
                    .map(|row| row.to_vec())
                    .to_vec()
            ),
            4
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::latest_day_to_cross(
                2,
                2,
                [[1, 1], [1, 2], [2, 1], [2, 2]]
                    .map(|row| row.to_vec())
                    .to_vec()
            ),
            4
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::latest_day_to_cross(
                3,
                3,
                [
                    [1, 2],
                    [2, 1],
                    [3, 3],
                    [2, 2],
                    [1, 1],
                    [1, 3],
                    [2, 3],
                    [3, 2],
                    [3, 1]
                ]
                .map(|row| row.to_vec())
                .to_vec()
            ),
            4
        );
    }
}
