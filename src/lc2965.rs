struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len() as i32;
        let mut xor_all = (1..=n.pow(2)).fold(0, |result, elem| result ^ elem);
        for line in grid {
            for number in line.iter() {
                xor_all ^= *number;
            }
        }
        let low_bit = xor_all ^ -xor_all;
        let mut repeated = 0;

        // assert_eq!(occured.len(), 1);
        // let missing = occured.into_iter().next().unwrap();
        let missing = 0;
        vec![repeated, missing]
    }
}
