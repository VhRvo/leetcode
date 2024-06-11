struct Solution;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::mem;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        Self::general_maximum_length(s, 3)
    }
    fn general_maximum_length(s: String, k: usize) -> i32 {
        let mut s_iter = s.chars();
        let mut char_lengths_map = HashMap::new();
        let mut process = |ch, mut current_length| {
            let lengths = char_lengths_map.entry(ch).or_insert(BinaryHeap::new());
            lengths.push(Reverse(current_length));
            while lengths.len() > k {
                lengths.pop();
            }
        };
        {
            let mut start = s_iter.next().unwrap();
            let mut length = 1;
            for ch in s_iter {
                if ch != start {
                    process(start, length);
                    start = ch;
                    length = 1;
                } else {
                    length += 1;
                }
            }
            process(start, length);
        }
        let result = char_lengths_map.iter_mut().fold(0, |result, (_, lengths)| {
            let mut length_stat: HashMap<usize, usize> = HashMap::new();
            let sorted_vec = mem::take(lengths).into_sorted_vec();
            for (&Reverse(length), index) in sorted_vec.iter().zip((0..k).rev()) {
                for count in 0..=index.min(length - 1) {
                    *length_stat.entry(length - count).or_default() += count + 1;
                }
            }
            length_stat
                .iter()
                .fold(0, |result, (&length, &count)| {
                    if count >= 3 {
                        result.max(length)
                    } else {
                        result
                    }
                })
                .max(result)
        });
        if result == 0 {
            -1
        } else {
            result as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::maximum_length("aaaa".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::maximum_length("abcdef".to_string()), -1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::maximum_length("abcaba".to_string()), 1);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::maximum_length("bbc".to_string()), -1);
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::maximum_length("aada".to_string()), 1);
    }

    #[test]
    fn test6() {
        assert_eq!(
            Solution::maximum_length("abbbbbggggggyyyggggg".to_string()),
            5
        );
    }

    #[test]
    fn test7() {
        assert_eq!(
            Solution::maximum_length(
                "cccerrrecdcdccedecdcccddeeeddcdcddedccdceeedccecde".to_string()
            ),
            2
        );
    }
}
