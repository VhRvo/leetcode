struct Solution;

impl Solution {
    pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        10 * (main_tank
            + additional_tank.min((((main_tank - 5) as f64) / 4.0_f64).floor() as i32 + 1))
    }
}
