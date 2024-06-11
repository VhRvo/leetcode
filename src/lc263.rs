struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n.is_positive() {
            let mut n = n;
            let prime_factors = [2, 3, 5];
            for prime_factor in prime_factors {
                while n % prime_factor == 0 {
                    n /= prime_factor;
                }
            }
            n == 1
        } else {
            false
        }
    }
}
