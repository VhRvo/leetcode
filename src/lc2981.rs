struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut map: HashMap<(char, i32), usize> = HashMap::new();
        let mut s_iter = s.chars().peekable();
        let mut start = *s_iter.peek().unwrap();
        let mut length = 0;
        for ch in s_iter {
            if ch != start {
                start = ch;
                length = 1;
            } else {
                length += 1;
            }
            *map.entry((ch, length)).or_default() += 1;
        }
        map.iter().fold(-1, |result, ((_, length), times)| {
            if *times >= 3 {
                result.max(*length)
            } else {
                result
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::lc2981::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::maximum_length("aaaa".to_string()), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::maximum_length("abcdef".to_string()), -1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::maximum_length("abcaba".to_string()), 1);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::maximum_length(
                "cccerrrecdcdccedecdcccddeeeddcdcddedccdceeedccecde".to_string()
            ),
            2
        );
    }
}
