struct Solution;
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n.is_negative() {
            false
        } else {
            let mut power_of_three = 1_u64;
            let n = n as u64;
            while power_of_three < n {
                power_of_three *= 3;
            }
            if power_of_three == n {
                true
            } else {
                false
            }
        }
    }
}
