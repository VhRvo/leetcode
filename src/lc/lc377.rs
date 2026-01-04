struct Solution;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::slice::Iter;

impl Solution {
    pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
        // nums.sort_by(|left, right| right.cmp(left));
        nums.sort();

        backtracking(nums.iter(), &mut Vec::new(), 0, target)
    }
}

#[test]
fn test_combination_sum4() {
    // println!("{}", Solution::combination_sum4(vec![1, 2, 3], 32));
    // println!("{}", Solution::combination_sum4(vec![2, 3], 5));
    // println!("{}", Solution::combination_sum4(vec![2, 3, 5], 15));
    println!(
        "{}",
        Solution::combination_sum4(
            vec![
                10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180,
                190, 200, 210, 220, 230, 240, 250, 260, 270, 280, 290, 300, 310, 320, 330, 340,
                350, 360, 370, 380, 390, 400, 410, 420, 430, 440, 450, 460, 470, 480, 490, 500,
                510, 520, 530, 540, 550, 560, 570, 580, 590, 600, 610, 620, 630, 640, 650, 660,
                670, 680, 690, 700, 710, 720, 730, 740, 750, 760, 770, 780, 790, 800, 810, 820,
                830, 840, 850, 860, 870, 880, 890, 900, 910, 920, 930, 940, 950, 960, 970, 980,
                990, 111
            ],
            999
        )
    );
}

fn combination(k: i32, n: i32) -> i32 {
    let n = n as i64;
    let k = k as i64;

    if n == 0 || k == 0 {
        1
    } else {
        let mut result = 1;
        for value in n - k + 1..=n {
            result *= value
        }
        for value in 1..=k {
            result /= value
        }
        result as i32
    }
}

fn combination_sum(sequence: &Vec<i32>) -> i32 {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    for num in sequence {
        *hash_map.entry(*num).or_insert(0) += 1;
    }
    let mut values = hash_map.values().map(|value| *value).collect::<Vec<_>>();
    values.sort();
    values.reverse();
    // println!("{:?}", sequence);
    // println!("{:?}", values);

    let mut iter = values.into_iter();
    let mut result = 1;
    let mut nums = iter.next().unwrap();
    for value in iter {
        result *= (1..=value)
            .map(|group| combination(group, nums + 1) * combination(group - 1, value - 1))
            .sum::<i32>();
        nums += value;
    }
    // println!("{:?}", result);
    result
}

fn backtracking(nums: Iter<i32>, sequence: &mut Vec<i32>, sum: i32, target: i32) -> i32 {
    match sum.cmp(&target) {
        Ordering::Greater => 0,
        Ordering::Equal => combination_sum(sequence),
        Ordering::Less => {
            let mut next = nums.clone();
            match next.next() {
                None => 0,
                Some(&num) => {
                    ({
                        sequence.push(num);
                        let result = backtracking(nums, sequence, sum + num, target);
                        sequence.pop();
                        result
                    }) + backtracking(next, sequence, sum, target)
                }
            }
        }
    }
}

#[test]
fn test_combination_sum() {
    println!("{}", combination_sum(&vec![1, 1, 2, 2]));
    println!("{}", combination_sum(&vec![1, 1, 2]));
}
#[test]
fn test_combination() {
    println!("{}", combination(0, 3));
    println!("{}", combination(2, 4));
}
