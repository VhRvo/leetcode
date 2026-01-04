struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut value = num;
        while value >= 10 {
            let mut result = 0;
            while value != 0 {
                result += value % 10;
                value /= 10;
            }
            value = result;
        }
        value
    }
}
