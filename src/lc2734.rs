struct Solution;
impl Solution {
    pub fn smallest_string(mut str: String) -> String {
        let bytes = unsafe { str.as_bytes_mut() };
        let mut ii = 0;
        let length = bytes.len();
        while ii < length {
            if bytes[ii] != b'a' {
                break;
            }
            ii += 1;
        }
        let start = ii;
        while ii < length {
            if bytes[ii] == b'a' {
                break;
            }
            bytes[ii] -= 1;
            ii += 1;
        }
        if start == ii {
            if let Some(byte) = bytes.last_mut() {
                *byte = b'z';
            }
        }
        str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::smallest_string("cbabc".to_string()),
            "baabc".to_string()
        );
        assert_eq!(
            Solution::smallest_string("aa".to_string()),
            "az".to_string()
        );
        assert_eq!(
            Solution::smallest_string("aaaa".to_string()),
            "aaaz".to_string()
        );
        assert_eq!(
            Solution::smallest_string("acbbc".to_string()),
            "abaab".to_string()
        );
        assert_eq!(
            Solution::smallest_string("leetcode".to_string()),
            "kddsbncd".to_string()
        );
    }
}
