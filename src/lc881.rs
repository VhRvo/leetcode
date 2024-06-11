struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort();
        let mut lo = 0;
        let mut hi = people.len() - 1;
        let mut result = 0;
        while lo < hi {
            result += 1;
            let rest = limit - people[hi];
            hi -= 1;
            if rest >= people[lo] {
                lo += 1;
            }
        }
        if lo == hi {
            result += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test1() {
        assert_eq!(Solution::num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::num_rescue_boats(vec![1, 2], 3), 1);
    }
}
