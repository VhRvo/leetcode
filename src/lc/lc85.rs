struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        Self::dp_monotone_stack(matrix)
    }
    fn insert_to_monotone_stack(
        stack: &mut Vec<(i32, i32)>,
        result: &mut i32,
        row: i32,
        column: i32,
    ) {
        while let Some(top) = stack.pop() {
            if top.0 < row && top.1 < column {
                continue;
            } else if top.0 >= row && top.1 >= column {
                stack.push(top);
                return;
            } else {
                stack.push(top);
                break;
            }
        }
        *result = (*result).max(row * column);
        stack.push((row, column));
    }
    fn dp_monotone_stack(matrix: Vec<Vec<char>>) -> i32 {
        use std::mem;
        let rows = matrix.len();
        let columns = matrix[0].len();
        let mut dp = vec![vec![((0, 0), Vec::new()); columns + 1]; 2];
        let mut result = 0;
        for ii in (0..rows).rev() {
            let current = ii % 2;
            let next = (ii + 1) % 2;
            for jj in (0..columns).rev() {
                if matrix[ii][jj] == '1' {
                    let down = dp[next][jj].0 .0 + 1;
                    let right = dp[current][jj + 1].0 .1 + 1;
                    let mut stack = mem::take(&mut dp[current][jj].1);
                    // If we use rolling array, we have to clear the stack here
                    // because dp[current][jj].2 may contain data from previous row
                    stack.clear();
                    stack.push((1, right));
                    result = result.max(right);
                    // should not use chain to combine (down, 1) in the loop below
                    // because the loop process row and column
                    for (row, column) in dp[next][jj + 1].1.iter() {
                        let row = (row + 1).min(down);
                        let column: i32 = (column + 1).min(right);
                        Self::insert_to_monotone_stack(&mut stack, &mut result, row, column);
                    }
                    Self::insert_to_monotone_stack(&mut stack, &mut result, down, 1);
                    dp[current][jj] = ((down, right), stack);
                } else {
                    dp[current][jj].0 = (0, 0);
                    dp[current][jj].1.clear();
                }
            }
        }
        result
    }
    fn dp_btree_map(matrix: Vec<Vec<char>>) -> i32 {
        use std::collections::BTreeMap;
        let rows = matrix.len();
        let columns = matrix[0].len();
        let mut dp = vec![vec![(0, 0, BTreeMap::<i32, i32>::new()); columns + 1]; 2];
        let mut result = 0;
        for ii in (0..rows).rev() {
            let current = ii % 2;
            let next = (ii + 1) % 2;
            // let current = ii;
            // let next = ii + 1;
            for jj in (0..columns).rev() {
                let mut map = BTreeMap::new();
                if matrix[ii][jj] == '1' {
                    let down = dp[next][jj].0 + 1;
                    let right = dp[current][jj + 1].1 + 1;
                    for (row, column) in dp[next][jj + 1].2.iter() {
                        let row = *row + 1;
                        let column = *column + 1;
                        map.entry(row.min(down))
                            .and_modify(|value: &mut i32| *value = (*value).max(column))
                            .or_insert(column);
                    }
                    for (row, column) in [(down, 1), (1, right)] {
                        map.entry(row)
                            .and_modify(|value: &mut i32| {
                                *value = (*value).max(column);
                            })
                            .or_insert(column);
                    }
                    for (row, column) in map.iter_mut() {
                        *column = (*column).min(right);
                        result = result.max(*row * *column);
                    }
                    dp[current][jj] = (down, right, map);
                } else {
                    dp[current][jj] = (0, 0, BTreeMap::new());
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
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::maximal_rectangle(vec![vec!['0']]), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::maximal_rectangle(vec![vec!['1']]), 1);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '1', '1'],
                vec!['0', '1', '0', '1', '0'],
                vec!['1', '1', '0', '1', '1'],
                vec!['1', '1', '0', '1', '1'],
                vec!['0', '1', '1', '1', '1']
            ]),
            6
        );
    }
}
