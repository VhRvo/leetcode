struct Solution;

impl Solution {
    fn longest_consecutive_segment_sentinel(bars: &[i32]) -> i32 {
        let mut prev = i32::MIN;
        let mut max_len = 0;
        let mut cur_len = 0;
        // Append a sentinel value to force final segment update
        for &bar in bars.iter().chain(std::iter::once(&i32::MAX)) {
            if prev + 1 == bar {
                cur_len += 1;
            } else {
                max_len = max_len.max(cur_len);
                cur_len = 1;
            }
            prev = bar;
        }
        max_len + 1
    }
    fn longest_consecutive_segment(bars: &[i32]) -> i32 {
        let mut prev = i32::MIN;
        let mut max_len = 0;
        let mut cur_len = 0;
        for &bar in bars {
            if prev + 1 == bar {
                cur_len += 1;
            } else {
                cur_len = 1;
            }
            max_len = max_len.max(cur_len);
            prev = bar;
        }
        max_len + 1
    }
    fn longest_consecutive_segment_chunk(bars: &[i32]) -> i32 {
        bars.chunk_by(|prev, curr| prev + 1 == *curr)
            .map(|chunk| chunk.len() as i32)
            .max()
            .unwrap_or(0)
            + 1
    }
    pub fn maximize_square_hole_area(
        _: i32,
        _: i32,
        mut h_bars: Vec<i32>,
        mut v_bars: Vec<i32>,
    ) -> i32 {
        h_bars.sort();
        v_bars.sort();
        let h_max = Self::longest_consecutive_segment_chunk(&h_bars);
        let v_max = Self::longest_consecutive_segment_chunk(&v_bars);
        let minimal = h_max.min(v_max);
        minimal * minimal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::maximize_square_hole_area(2, 1, vec![2, 3], vec![2]),
            4
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::maximize_square_hole_area(1, 1, vec![2], vec![2]),
            4
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::maximize_square_hole_area(2, 3, vec![2, 3], vec![2, 4]),
            4
        )
    }
}
