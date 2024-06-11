struct Solution;

use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut hash_map = HashMap::new();
        let mut queue = VecDeque::new();
        for (ii, ch) in s.chars().enumerate() {
            match hash_map.entry(ch) {
                Entry::Occupied(mut entry) => {
                    entry.insert(-1);
                    while let Some((front, _)) = queue.front() {
                        if let Some(-1) = hash_map.get(front).copied() {
                            queue.pop_front();
                        } else {
                            break;
                        }
                    }
                }
                Entry::Vacant(entry) => {
                    let ii = ii as i32;
                    entry.insert(ii);
                    queue.push_back((ch, ii));
                }
            }
        }
        queue.front().map(|pair| pair.1).unwrap_or(-1)
    }

    // pub fn first_uniq_char(s: String) -> i32 {
    //     let mut hash_map = HashMap::new();
    //     let mut queue = VecDeque::new();
    //     for (ii, ch) in s.chars().enumerate() {
    //         let current = *hash_map
    //             .entry(ch)
    //             .and_modify(|prev| {
    //                 *prev = -1;
    //             })
    //             .or_insert_with(|| {
    //                 let ii = ii as i32;
    //                 queue.push_back((ch, ii));
    //                 ii
    //             });
    //         if current == -1 {
    //             while let Some((front, _)) = queue.front() {
    //                 if let Some(-1) = hash_map.get(front).copied {
    //                     queue.pop_front();
    //                 } else {
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    //     queue.front().map(|pair| pair.1).unwrap_or(-1)
    // }
    // pub fn first_uniq_char(s: String) -> i32 {
    //     let mut hash_map = HashMap::new();
    //     for (ii, ch) in s.chars().enumerate() {
    //         hash_map
    //             .entry(ch)
    //             .and_modify(|prev| *prev = i32::MAX)
    //             .or_insert(ii as i32);
    //     }
    //     let front = *hash_map.values().min().unwrap();
    //     if i32::MAX == front {
    //         -1
    //     } else {
    //         front
    //     }
    // }
    // pub fn first_uniq_char(s: String) -> i32 {
    //     let hash_map = s.chars().fold(HashMap::new(), |mut acc, ch| {
    //         *acc.entry(ch).or_insert(0) += 1;
    //         acc
    //     });
    //     for (ii, ch) in s.chars().enumerate() {
    //         if Some(1) == hash_map.get(&ch).copied {
    //             return ii as i32;
    //         }
    //     }
    //     -1
    // }
}

#[test]
fn test() {
    let diff = b'b' - b'a';
    println!("{diff}");
    assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0)
}
