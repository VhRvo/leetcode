struct Solution;

use std::collections::BTreeSet;
use std::collections::HashSet;
const MODULO: i64 = 1_000_000_007;
impl Solution {
    fn segment_lengths(len: i32, mut fences: Vec<i32>) -> Vec<i32> {
        fences.push(1);
        fences.push(len);
        fences.sort();
        fences
            .windows(2)
            .map(|segment| segment[1] - segment[0])
            .collect()
    }
    fn get_prefix_sum(lengths: Vec<i32>) -> Vec<i32> {
        let mut prefix_sum = vec![0; 1 + lengths.len()];
        for (ii, length) in lengths.into_iter().enumerate() {
            prefix_sum[ii + 1] = prefix_sum[ii] + length;
        }
        prefix_sum
    }
    fn get_interval_sum_from_prefix_sum(prefix_sum: &[i32]) -> HashSet<i32> {
        let mut interval_sum = HashSet::with_capacity(prefix_sum.len() * prefix_sum.len());
        for start in 0..prefix_sum.len() {
            for end in start + 1..prefix_sum.len() {
                let length = prefix_sum[end] - prefix_sum[start];
                interval_sum.insert(length);
            }
        }
        interval_sum
    }
    fn get_interval_sum_from_fences(len: i32, mut fences: Vec<i32>) -> HashSet<i32> {
        fences.push(1);
        fences.push(len);
        fences.sort();

        let mut interval_sum = HashSet::new();
        for start in 0..fences.len() {
            for end in start + 1..fences.len() {
                interval_sum.insert(fences[end] - fences[start]);
            }
        }
        interval_sum
    }
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        Self::one_step(m, n, h_fences, v_fences)
        // Self::two_steps(m, n, h_fences, v_fences)
    }
    fn one_step(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        let h_interval_sum = Self::get_interval_sum_from_fences(m, h_fences);
        let v_interval_sum = Self::get_interval_sum_from_fences(n, v_fences);
        if let Some(&max_length) = h_interval_sum.intersection(&v_interval_sum).max() {
            let max_length = max_length as i64;
            ((max_length * max_length) % MODULO) as i32
        } else {
            -1
        }
    }
    fn two_steps(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        let h_segment_lengths = Self::segment_lengths(m, h_fences);
        let v_segment_lengths = Self::segment_lengths(n, v_fences);
        let h_prefix_sum = Self::get_prefix_sum(h_segment_lengths);
        let v_prefix_sum = Self::get_prefix_sum(v_segment_lengths);
        let h_interval_sum = Self::get_interval_sum_from_prefix_sum(&h_prefix_sum);
        let mut max_length = 0;
        for start in 0..v_prefix_sum.len() {
            for end in start + 1..v_prefix_sum.len() {
                let length = v_prefix_sum[end] - v_prefix_sum[start];
                if h_interval_sum.contains(&length) {
                    max_length = max_length.max(length);
                }
            }
        }
        if max_length == 0 {
            -1
        } else {
            let max_length = max_length as i64;
            ((max_length * max_length) % MODULO) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::maximize_square_area(4, 3, vec![2, 3], vec![2]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::maximize_square_area(6, 7, vec![2], vec![4]), -1);
    }
}
