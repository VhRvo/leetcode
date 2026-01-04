struct Solution;

impl Solution {
    pub fn base_neg2(n: i32) -> String {
        let mut n = n as i64;
        let mut carry = 0;
        let mut weight_sign = 1_i8;
        let mut bits = String::with_capacity((1 + n).ilog2() as usize);
        loop {
            let bit = (n & 1) as i8;
            match bit + carry {
                -1 => {
                    assert!(weight_sign.is_negative());
                    bits.push('1');
                    carry = 1;
                }
                0 => {
                    bits.push('0');
                    carry = 0;
                }
                1 => {
                    bits.push('1');
                    carry = 0;
                }
                2 => {
                    assert!(weight_sign.is_positive());
                    bits.push('0');
                    carry = -1;
                }
                _ => unreachable!(),
            }
            carry += if bit == 1 && weight_sign.is_negative() {
                1
            } else {
                0
            };

            n >>= 1;
            weight_sign *= -1;

            if n == 0 && carry == 0 {
                break;
            }
        }
        bits.chars().rev().collect()
    }
}

#[test]
fn test() {
    // Solution::base_neg2(24);
    // println!("{}", Solution::base_neg2(30));
    assert_eq!(Solution::base_neg2(0), "0".to_string());
    assert_eq!(Solution::base_neg2(2), "110".to_string());
    assert_eq!(Solution::base_neg2(3), "111".to_string());
    assert_eq!(Solution::base_neg2(4), "100".to_string());
    assert_eq!(Solution::base_neg2(10), "11110".to_string());
    assert_eq!(Solution::base_neg2(30), "1100010".to_string());
}
