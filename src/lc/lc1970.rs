use std::vec;

struct Solution;

impl Solution {
    fn contains(x: usize, y: usize, row: usize, col: usize) -> bool {
        1 <= x && x <= row && 1 <= y && y <= col
    }
    fn spread_modification(
        connection: &mut [Vec<i32>],
        row_count: &mut [usize],
        row: usize,
        col: usize,
        x: usize,
        y: usize,
    ) -> bool {
        if connection[x][y] == 0 {
            row_count[x] += 1;
            if row_count[x] == col {
                return true;
            }
            // 4 directions
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;
                if Self::contains(new_x, new_y, row, col) {
                    if connection[new_x][new_y] != 0 {
                        connection[new_x][new_y] -= 1;
                        if Self::spread_modification(connection, row_count, row, col, new_x, new_y)
                        {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
    fn print_maze(maze: &[Vec<i32>]) {
        for row in maze.iter().skip(1) {
            println!("{:?}", &row[1..]);
        }
        println!();
    }
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let row = row as usize;
        let col = col as usize;
        let mut connection = {
            let mut connection = vec![vec![0; col + 1]; row + 1];
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
        let mut maze = vec![vec![0; col + 1]; row + 1];
        for cell in cells.iter() {
            let x = cell[0] as usize;
            let y = cell[1] as usize;
            maze[x][y] = 1;
            println!("Day: {}", result + 1);
            Self::print_maze(&maze);
            if connection[x][y] != 0 {
                connection[x][y] = 0;
                if Self::spread_modification(&mut connection, &mut row_count, row, col, x, y) {
                    break;
                }
            }
            println!("{:?}", row_count);
            Self::print_maze(&connection);
            // println!("{:?}", connection);
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

    #[test]
    fn test4() {
        assert_eq!(
            Solution::latest_day_to_cross(
                13,
                9,
                [
                    [12, 6],
                    [3, 4],
                    [2, 9],
                    [9, 4],
                    [9, 2],
                    [6, 4],
                    [4, 4],
                    [8, 6],
                    [4, 9],
                    [5, 6],
                    [7, 5],
                    [12, 4],
                    [11, 8],
                    [3, 7],
                    [2, 6],
                    [9, 8],
                    [3, 5],
                    [13, 4],
                    [1, 3],
                    [10, 2],
                    [8, 9],
                    [6, 6],
                    [11, 7],
                    [11, 1],
                    [13, 9],
                    [12, 7],
                    [10, 7],
                    [8, 2],
                    [1, 8],
                    [7, 3],
                    [6, 5],
                    [2, 1],
                    [10, 6],
                    [4, 8],
                    [4, 2],
                    [9, 7],
                    [6, 2],
                    [3, 6],
                    [12, 2],
                    [10, 3],
                    [10, 5],
                    [9, 5],
                    [8, 8],
                    [8, 7],
                    [3, 2],
                    [13, 6],
                    [3, 1],
                    [5, 1],
                    [2, 7],
                    [8, 3],
                    [12, 5],
                    [11, 2],
                    [6, 3],
                    [1, 4],
                    [13, 3],
                    [4, 1],
                    [9, 9],
                    [7, 7],
                    [4, 3],
                    [12, 1],
                    [2, 2],
                    [7, 6],
                    [4, 6],
                    [7, 9],
                    [7, 2],
                    [3, 8],
                    [1, 6],
                    [11, 3],
                    [11, 4],
                    [5, 9],
                    [13, 8],
                    [1, 9],
                    [10, 1],
                    [9, 1],
                    [6, 1],
                    [10, 9],
                    [12, 9],
                    [11, 5],
                    [8, 1],
                    [13, 5],
                    [9, 6],
                    [13, 2],
                    [6, 8],
                    [2, 8],
                    [5, 3],
                    [3, 3],
                    [13, 1],
                    [11, 9],
                    [9, 3],
                    [2, 4],
                    [5, 2],
                    [8, 5],
                    [13, 7],
                    [12, 8],
                    [5, 5],
                    [7, 1],
                    [7, 4],
                    [2, 5],
                    [6, 9],
                    [4, 7],
                    [5, 8],
                    [1, 5],
                    [10, 8],
                    [8, 4],
                    [1, 1],
                    [3, 9],
                    [1, 2],
                    [7, 8],
                    [1, 7],
                    [6, 7],
                    [11, 6],
                    [4, 5],
                    [5, 7],
                    [2, 3],
                    [10, 4],
                    [5, 4],
                    [12, 3]
                ]
                .map(|row| row.to_vec())
                .to_vec()
            ),
            35
        );
    }
}
