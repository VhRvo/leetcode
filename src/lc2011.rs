struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut result = 0;
        for operation in operations {
            if operation.contains("++") {
                result += 1;
            } else {
                result -= 1;
            }
        }
        result
    }
}
