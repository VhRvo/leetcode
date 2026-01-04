struct Solution;

pub(crate) fn main() {
    println!("{}", Solution::nth_ugly_number(10));
}

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::{BTreeSet, BinaryHeap};

        let n = n as i64;
        let prime_factors = [2, 3, 5];
        let mut heap = BinaryHeap::from([Reverse(1_i64)]);
        let mut occured = BTreeSet::from([1]);
        let mut index = 1;
        loop {
            let Reverse(top) = heap.pop().unwrap();
            if index == n {
                return top as i32;
            } else {
                index += 1;
                for prime_factor in prime_factors {
                    let value = prime_factor * top;
                    if !occured.contains(&value) {
                        heap.push(Reverse(value));
                        occured.insert(value);
                    }
                }
            }
        }
    }
}
