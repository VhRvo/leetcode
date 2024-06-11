use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix_sum = 0;
        let mut prefix_sums = Vec::new();
        for num in nums {
            prefix_sum += num % k;
            prefix_sum %= k;
            prefix_sums.push(prefix_sum);
        }

        let mut result = 0;
        let mut occupied = HashMap::new();
        for prefix_sum in prefix_sums {
            
        }
        
        ; todo!()
    }
}
