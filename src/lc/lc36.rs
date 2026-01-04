struct Solution;

const BLOCK_INDICES: &[(usize, usize)] = &[
    (0, 0),
    (0, 1),
    (0, 2),
    (1, 0),
    (1, 1),
    (1, 2),
    (2, 0),
    (2, 1),
    (2, 2),
];
const BLOCK_STARTS: &[(usize, usize)] = &[
    (0, 0),
    (0, 3),
    (0, 6),
    (3, 0),
    (3, 3),
    (3, 6),
    (6, 0),
    (6, 3),
    (6, 6),
];

impl Solution {
    fn is_row_valid(digits: &[char]) -> bool {
        let mut bit_set: u16 = 0;
        for &digit in digits {
            if digit.is_numeric() {
                let mask: u16 = 1 << digit.to_digit(10).unwrap();
                if (bit_set & mask) != 0 {
                    return false;
                }
                bit_set |= mask;
            }
        }
        true
    }
    fn is_column_valid(board: &[Vec<char>], no_column: usize) -> bool {
        let column = (0..9)
            .map(|no_row| board[no_row][no_column])
            .collect::<Vec<_>>();
        Self::is_row_valid(&column)
    }
    fn is_block_valid(board: &[Vec<char>], (row_offset, column_offset): (usize, usize)) -> bool {
        let block = BLOCK_INDICES
            .iter()
            .map(|&(no_row, no_column)| board[no_row + row_offset][no_column + column_offset])
            .collect::<Vec<_>>();
        Self::is_row_valid(&block)
    }
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::convert::identity;
        let all_rows_valid = board
            .iter()
            .map(|line| Self::is_row_valid(line))
            .all(identity);
        let all_columns_valid = (0..9)
            .map(|no_column| Self::is_column_valid(&board, no_column))
            .all(identity);
        let all_blocks_valid = BLOCK_STARTS
            .iter()
            .map(|pair| Self::is_block_valid(&board, *pair))
            .all(identity);

        all_rows_valid && all_columns_valid && all_blocks_valid
    }
    // pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    //     fn insert(bit_set: &mut u16, digit: u16) -> bool {
    //         let mask = 1_u16 << digit;
    //         let result = *bit_set & mask != 0;
    //         *bit_set |= mask;
    //         result
    //     }
    //
    //     let mut rows = [0_u16; 9];
    //     let mut columns = [0_u16; 9];
    //     let mut blocks = [0_u16; 9];
    //
    //     for (no_row, row) in board.iter().enumerate() {
    //         for (no_col, ch) in row.iter().enumerate() {
    //             if !ch.is_numeric() {
    //                 continue;
    //             }
    //
    //             let block_index = no_row / 3 * 3 + no_col / 3;
    //             let digit = ch.to_digit(10).unwrap() as u16;
    //
    //             if insert(&mut rows[no_row], digit)
    //                 || insert(&mut columns[no_col], digit)
    //                 || insert(&mut blocks[block_index], digit)
    //             {
    //                 return false;
    //             }
    //         }
    //     }
    //     true
    // }
}

#[test]
fn test_is_row_valid() {
    println!("{:?}", Solution::is_row_valid(&['1', '1']));
}

#[test]
fn test() {
    println!(
        "{:?}",
        ('0'..='9')
            .map(|ch| ch.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    );
    // [
    //     ["8", "3", ".", ".", "7", ".", ".", ".", "."],
    //     ["6", ".", ".", "1", "9", "5", ".", ".", "."],
    //     [".", "9", "8", ".", ".", ".", ".", "6", "."],
    //     ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
    //     ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
    //     ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
    //     [".", "6", ".", ".", ".", ".", "2", "8", "."],
    //     [".", ".", ".", "4", "1", "9", ".", ".", "5"],
    //     [".", ".", ".", ".", "8", ".", ".", "7", "9"],
    // ];
}
