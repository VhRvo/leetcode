struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        // let rows = matrix.len();
        let columns = matrix.first().unwrap().len();
        let mut last_line = vec![0; columns];
        let mut bh = BinaryHeap::new();
        for row in matrix.iter() {
            let mut current_line_xor_value = 0;
            for (elem, last_line_xor_value) in row.iter().zip(last_line.iter_mut()) {
                current_line_xor_value ^= elem;
                *last_line_xor_value ^= current_line_xor_value;
                bh.push(Reverse(*last_line_xor_value));
                while bh.len() > k as usize {
                    bh.pop();
                }
            }
        }
        bh.peek().unwrap().0
    }
}

#[test]
fn test1() {
    let mut bh: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    bh.push(Reverse(8));
    bh.push(Reverse(2));
    println!("{:?}", bh.peek());
}

#[test]
fn test2() {
    println!("{}", 5 ^ 2);
    println!("{}", 5 ^ 1);
}

#[test]
fn test3() {
    println!(
        "{}",
        Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 1)
    );
    println!(
        "{}",
        Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 2)
    );
    println!(
        "{}",
        Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 3)
    );
}
