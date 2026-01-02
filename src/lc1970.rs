struct Solution;

impl Solution {
    fn update_upwards(
        connection: &mut [Vec<i32>],
        row_count: &mut [usize],
        col: usize,
        x: usize,
        y: usize,
    ) -> bool {
        if connection[x][y] == 0 {
            row_count[x] += 1;
            if row_count[x] == col {
                return true;
            }
            // upwards
            if x != 0 {
                let new_x = x - 1;
                let new_y = y;
                if connection[new_x][new_y] != 0 {
                    connection[new_x][new_y] -= 1;
                    // if connection[new_x][new_y] == 0 {
                    //     row_count[new_x] += 1;
                    //     if row_count[new_x] == col {
                    //         return true;
                    //     }
                    // }
                    return Self::update_upwards(connection, row_count, col, new_x, new_y);
                }
            }
        }
        return false;
    }
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let row = row as usize;
        let col = col as usize;
        let contains = |x: usize, y: usize| 1 <= x && x <= row && 1 <= y && y <= col;
        let mut connection = {
            let mut connection = vec![vec![0; col + 1]; row + 1];
            for jj in 1..=col {
                connection[0][jj] = 1;
            }
            for ii in 1..=row {
                connection[ii][1] = 2;
                connection[ii][col] = 2;
                for jj in 2..col {
                    connection[ii][jj] = 3;
                }
            }
            connection
        };
        println!("{:?}", connection);
        let mut row_count = vec![0; row + 1];
        let mut result = 0;
        'outer: for cell in cells.iter() {
            let x = cell[0] as usize;
            let y = cell[1] as usize;
            if connection[x][y] != 0 {
                connection[x][y] = 0;
                row_count[x] += 1;
                if row_count[x] == col {
                    break 'outer;
                }
            }
            // left & right
            for (x_offset, y_offset) in [(0, 1), (0, -1)] {
                let new_x = (x as isize + x_offset) as usize;
                let new_y = (y as isize + y_offset) as usize;
                if !contains(new_x, new_y) {
                    continue;
                }
                if connection[new_x][new_y] == 0 {
                    continue;
                }
                connection[new_x][new_y] -= 1;
                if Self::update_upwards(&mut connection, &mut row_count, col, new_x, new_y) {
                    break 'outer;
                }
            }
            {
                // up
                let new_x = x - 1;
                let new_y = y;
                if connection[new_x][new_y] != 0 {
                    connection[new_x][new_y] -= 1;
                    if Self::update_upwards(&mut connection, &mut row_count, col, new_x, new_y) {
                        break 'outer;
                    }
                }
            }
            {
                // down
                let new_x = x + 1;
                let new_y = y;
                if contains(new_x, new_y) && connection[new_x][new_y] != 0 {
                    connection[new_x][new_y] -= 1;
                    if connection[new_x][new_y] == 0 {
                        row_count[new_x] += 1;
                        if row_count[x] == col {
                            break 'outer;
                        }
                    }
                }
            }
            println!("{:?}", connection);
            result += 1;
        }
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
            2
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
            1
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
            3
        );
    }
}
