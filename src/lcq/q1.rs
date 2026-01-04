struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len() * 2);
        for _ in 0..2 {
            for element in nums.iter() {
                result.push(*element);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 3]),
            vec![1, 2, 3, 1, 2, 3]
        )
    }
}
