struct Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SinglePattern {
    Char(u8),
    Dot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pattern {
    Single(SinglePattern),
    Star(SinglePattern),
}

fn try_parse(value: &str) -> Option<Vec<Pattern>> {
    let mut result = Vec::with_capacity(value.len() / 2);
    for ch in value.as_bytes().iter().copied() {
        if ch == b'.' {
            result.push(Pattern::Single(SinglePattern::Dot));
        } else if ch == b'*' {
            let last = result.pop()?;
            match last {
                Pattern::Single(single) => {
                    result.push(Pattern::Star(single));
                }
                Pattern::Star(_) => return None,
            }
        } else {
            result.push(Pattern::Single(SinglePattern::Char(ch)));
        }
    }
    Some(result)
}

impl Solution {
    pub fn is_match(mut s: String, mut p: String) -> bool {
        s.push('-');
        p.push('-');
        let patterns = match try_parse(&p) {
            Some(patterns) => patterns,
            None => return false,
        };
        let s = s.as_bytes();
        // let mut dp = vec![vec![false; s.len() + 1]; patterns.len() + 1];
        let mut dp = vec![vec![false; s.len() + 1]; 2];
        dp[patterns.len() % 2][s.len()] = true;
        for (ii, pattern) in patterns.iter().enumerate().rev() {
            let current = ii % 2;
            let next = (ii + 1) % 2;
            // IMPORTANT: When using a rolling array, we must reset dp[current][s.len()] to false.
            // Only the base case dp[patterns.len()][s.len()] should be true; all other rows
            // should have false at this boundary. Without this reset, after two iterations the
            // reused row still carries a stale `true`, producing wrong results.
            dp[current][s.len()] = false;
            for (jj, ch) in s.iter().enumerate().rev() {
                dp[current][jj] = match pattern {
                    Pattern::Single(single_pattern) => match single_pattern {
                        SinglePattern::Char(pattern) => pattern == ch && dp[next][jj + 1],
                        SinglePattern::Dot => dp[next][jj + 1],
                    },
                    Pattern::Star(single_pattern) => match single_pattern {
                        SinglePattern::Char(pattern) => {
                            (pattern == ch && (dp[current][jj + 1] || dp[next][jj + 1]))
                                || dp[next][jj]
                        }
                        SinglePattern::Dot => {
                            dp[current][jj + 1] || dp[next][jj + 1] || dp[next][jj]
                        }
                    },
                }
            }
        }
        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::is_match("b".to_string(), ".*a".to_string()),
            false
        );
    }
}
