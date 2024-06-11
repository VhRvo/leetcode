use std::cell::Cell;

struct Solution;
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let length = s.len();
        let mut suffix = vec![0; length];
        let mut prefix = vec![0; length];
        let chars = s.chars().collect::<Vec<_>>();
        if chars[0] == '0' {
            prefix[0] = 1;
        }
        if chars[length - 1] == '1' {
            suffix[length - 1] = 1;
        }
        let prefix_iter = Cell::from_mut(prefix.as_mut_slice())
            .as_slice_of_cells()
            .windows(2)
            .zip(chars.iter().skip(1));
        let suffix_iter = Cell::from_mut(suffix.as_mut_slice())
            .as_slice_of_cells()
            .windows(2)
            .rev()
            .zip(chars.iter().rev().skip(1));
        prefix_iter.zip(suffix_iter).for_each(
            |((prefix_pair, &prefix_char), (suffix_pair, &suffix_char))| {
                prefix_pair[1].set(prefix_pair[0].get() + (prefix_char == '0') as i32);
                suffix_pair[0].set(suffix_pair[1].get() + (suffix_char == '1') as i32);
            },
        );
        prefix
            .iter()
            .zip(suffix.iter().skip(1))
            .map(|(prefix_sum, suffix_sum)| prefix_sum + suffix_sum)
            .max()
            .unwrap()
    }
}
