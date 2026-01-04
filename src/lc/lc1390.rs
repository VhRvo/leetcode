struct Solution;

use std::collections::BTreeSet;
impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        const MAX: usize = 1e5 as usize + 5;
        let primes = {
            let mut primes = BTreeSet::new();
            let mut is_primes = vec![true; MAX];
            for number in 2..MAX {
                let is_prime = is_primes[number];
                if is_prime {
                    primes.insert(number as i32);
                }
                let mut multiple = number * 2;
                while multiple < MAX {
                    is_primes[multiple] = false;
                    multiple += number;
                }
            }
            primes
        };

        let mut result = 0;
        for num in nums.iter() {
            for prime in primes.iter() {
                if prime > num {
                    break;
                }
                let divisor = num / prime;
                if num % prime == 0 && primes.contains(&divisor) && divisor != *prime {
                    result += 1 + prime + divisor + num;
                    break;
                }
                if num % prime == 0 && prime * prime == divisor {
                    result += 1 + prime + divisor + num;
                    break;
                }
            }
        }
        result
    }
    pub fn brute_force(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        'outer: for num in nums.iter() {
            let mut factors = 0;
            let mut factors_sum = 0;
            for factor in 1..=*num {
                if num % factor == 0 {
                    factors += 1;
                    factors_sum += factor;
                    if factors == 5 {
                        continue 'outer;
                    }
                }
            }
            if factors == 4 {
                result += factors_sum;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::sum_four_divisors(vec![21, 4, 7]), 32);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::sum_four_divisors(vec![21, 21]), 64);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::sum_four_divisors(vec![1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::sum_four_divisors(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            45
        );
    }
}
