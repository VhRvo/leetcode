struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut prefix_sum = 0;
        let mut result = 0;
        let mut occupied: HashMap<i32, i32> = HashMap::new();
        occupied.insert(0, 1);
        for num in nums {
            prefix_sum += num % k + k;
            prefix_sum %= k;
            let count = occupied.entry(prefix_sum).or_default();
            result += *count;
            *count += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::subarrays_div_by_k(vec![5], 9), 0);
    }
    // #[test]
    // fn test3() {
    //     assert_eq!(Solution::subarrays_div_by_k(vec![], ), );
    // }
}
