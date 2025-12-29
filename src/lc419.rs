struct Solution;

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        for (ii, line) in board.iter().enumerate() {
            for (jj, cell) in line.iter().copied().enumerate() {
                if cell == 'X'
                    && (ii == 0 || board[ii - 1][jj] != 'X')
                    && (jj == 0 || board[ii][jj - 1] != 'X')
                {
                    result += 1;
                }
            }
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
            Solution::count_battleships(vec![
                vec!['X', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
            ]),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_battleships(vec![vec!['.'],]), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::count_battleships(vec![vec!['X', 'X', 'X'],]), 1);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::count_battleships(vec![vec!['.', '.'], vec!['X', 'X']]),
            1
        );
    }
}
