struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        Self::dp_monotone_stack(matrix)
    }
    fn insert_to_monotone_stack(
        stack: &mut Vec<(i32, i32)>,
        result: &mut i32,
        row_span: i32,
        column_span: i32,
    ) {
        // Pop elements that are strictly dominated by the new element
        while stack
            .last()
            .map_or(false, |top| row_span > top.0 && column_span > top.1)
        {
            stack.pop();
        }

        // Skip insertion if the top element dominates or equals the new element
        if stack
            .last()
            .map_or(false, |top| top.0 >= row_span && top.1 >= column_span)
        {
            return;
        }

        *result = (*result).max(row_span * column_span);
        stack.push((row_span, column_span));
    }
    fn dp_monotone_stack(matrix: Vec<Vec<char>>) -> i32 {
        use std::mem;
        let rows = matrix.len();
        let columns = matrix[0].len();
        let mut dp_starting_at = vec![vec![((0, 0), Vec::new()); columns + 1]; 2];
        let mut result = 0;
        for ii in (0..rows).rev() {
            let current = ii % 2;
            let next = (ii + 1) % 2;
            for jj in (0..columns).rev() {
                if matrix[ii][jj] == '1' {
                    let span_from_down = dp_starting_at[next][jj].0 .0 + 1;
                    let span_from_right = dp_starting_at[current][jj + 1].0 .1 + 1;
                    let mut stack = mem::take(&mut dp_starting_at[current][jj].1);
                    // Clear the stack when using rolling array since dp[current][jj]
                    // may contain stale data from the previous row iteration
                    stack.clear();
                    stack.push((1, span_from_right));
                    result = result.max(span_from_right);
                    // Process elements from diagonal cell; cannot chain (down, 1) here
                    // since the loop handles both row and column expansion
                    for (row_span, column_span) in dp_starting_at[next][jj + 1].1.iter() {
                        let row_span = (row_span + 1).min(span_from_down);
                        let column_span: i32 = (column_span + 1).min(span_from_right);
                        Self::insert_to_monotone_stack(
                            &mut stack,
                            &mut result,
                            row_span,
                            column_span,
                        );
                    }
                    Self::insert_to_monotone_stack(&mut stack, &mut result, span_from_down, 1);
                    dp_starting_at[current][jj] = ((span_from_down, span_from_right), stack);
                } else {
                    dp_starting_at[current][jj].0 = (0, 0);
                    dp_starting_at[current][jj].1.clear();
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
