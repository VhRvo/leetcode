struct Solution;

impl Solution {
    fn max_consective_segment(bars: &[i32]) -> i32 {
        let mut current = i32::MIN;
        let mut result = 0;
        let mut length = 0;
        for bar in bars.iter() {
            if current + 1 == *bar {
                current = *bar;
                length += 1;
            } else {
                result = result.max(length + 1);
                current = *bar;
                length = 1;
            }
        }
        result = result.max(length + 1);
        result
    }
    pub fn maximize_square_hole_area(
        _: i32,
        _: i32,
        mut h_bars: Vec<i32>,
        mut v_bars: Vec<i32>,
    ) -> i32 {
        h_bars.sort();
        v_bars.sort();
        let h_max = Self::max_consective_segment(&h_bars);
        let v_max = Self::max_consective_segment(&v_bars);
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
