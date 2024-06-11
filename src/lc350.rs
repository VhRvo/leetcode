struct Solution;

use std::cmp::Ordering;
use std::slice::Iter;

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort();
        nums2.sort();

        let mut result = Vec::new();
        merge(&mut result, nums1.iter(), nums2.iter());
        result
    }
}

fn merge(result: &mut Vec<i32>, mut left: Iter<i32>, mut right: Iter<i32>) {
    if let (Some(value1), Some(value2)) = (left.next(), right.next()) {
        match value1.cmp(value2) {
            Ordering::Less => {
                merge_already_known_one(result, *value2, right, left);
            }
            Ordering::Equal => {
                result.push(*value1);
                merge(result, left, right);
            }
            Ordering::Greater => merge_already_known_one(result, *value1, left, right),
        }
    }
}
fn merge_already_known_one(
    result: &mut Vec<i32>,
    value: i32,
    left: Iter<i32>,
    mut right: Iter<i32>,
) {
    if let Some(value2) = right.next() {
        match value.cmp(value2) {
            Ordering::Less => {
                merge_already_known_one(result, *value2, right, left);
            }
            Ordering::Equal => {
                result.push(value);
                merge(result, left, right);
            }
            Ordering::Greater => {
                merge_already_known_one(result, value, left, right);
            }
        }
    }
}
