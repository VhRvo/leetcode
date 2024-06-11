use std::ops::Mul;

struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        fn helper(mut x: i32) -> Option<i32> {
            let mut result = 0_i32;
            while (x) != 0 {
                result = result.checked_mul(10)?.checked_add(x % 10)?;
                x /= 10;
            }
            Some(result)
        }
        x.signum().mul(helper(x.abs()).unwrap_or_default())
    }
}

#[test]
fn test() {
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(0), 0);
    assert_eq!(Solution::reverse(1534236469), 0);
}
