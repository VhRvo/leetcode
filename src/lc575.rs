struct Solution;

impl Solution {
    pub fn distribute_candies(mut candy_type: Vec<i32>) -> i32 {
        let half = candy_type.len() / 2;
        candy_type.sort();
        candy_type.dedup();
        candy_type.len().min(half) as i32
        // use std::collections::HashSet; let half = candy_type.len() / 2;
        // let unique_candy: HashSet<i32> = candy_type.into_iter().collect();
        // half.min(unique_candy.len()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
        assert_eq!(Solution::distribute_candies(vec![6, 6, 6, 6]), 1);
    }
}
