struct Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        num_bottles
            + ((num_bottles - num_exchange) as f32 / (num_exchange - 1) as f32).floor() as i32
            + 1
    }
}
