struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        digits.reverse();
        let mut carry = 1;
        for digit in digits.iter_mut() {
            match *digit + carry {
                10 => {
                    carry = 1;
                    *digit = 0;
                }
                n => {
                    carry = 0;
                    *digit = n;
                }
            }
        }
        if carry == 1 {
            digits.push(1);
        }
        digits.reverse();
        digits
    }
}
