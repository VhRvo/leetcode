struct Solution;

impl Solution {
    fn calculate_area(line: i32, squares: &[Vec<i32>]) -> (i32, i32) {
        let mut below = 0;
        let mut above = 0;
        for sqaure in squares.iter() {
            let y = sqaure[1];
            let length = sqaure[2];
            let area = length * length;
            if y >= line {
                above += area;
            } else if y + length <= line {
                below += area;
            } else {
                below += (line - y) * length;
                above += (y + length - line) * length;
            }
        }
        (below, above)
    }
    fn calculate_line_area(line: i32, squares: &[Vec<i32>]) -> i32 {
        let mut area = 0;
        for sqaure in squares.iter() {
            let y = sqaure[1];
            let length = sqaure[2];
            // y <= line - 1 && y + length >= line
            if y < line && y + length >= line {
                area += length;
            }
        }
        area
    }
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let (lo, hi) = {
            let mut lowerest = i32::MAX;
            let mut upperest = i32::MIN;
            for square in squares.iter() {
                lowerest = lowerest.min(square[1]);
                upperest = upperest.max(square[1] + square[2]);
            }
            (lowerest, upperest)
        };
        let mi = (lo..hi + 1).collect::<Vec<_>>().partition_point(|&line| {
            let (below, above) = Self::calculate_area(line, &squares);
            below < above
        }) as i32
            + lo;
        let (below, above) = Self::calculate_area(mi, &squares);
        let area = Self::calculate_line_area(mi, &squares);
        if below == above {
            mi as f64
        } else {
            mi as f64 - ((below - above) as f64 / (2 * area) as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]]),
            1.0
        );
    }

    #[test]
    fn test2() {
        assert!(
            (Solution::separate_squares(vec![vec![0, 0, 2], vec![1, 1, 1]]) - 7.0 / 6.0).abs()
                < 1e-5,
        );
    }

    #[test]
    fn test3() {
        let result = Solution::separate_squares(vec![vec![23, 29, 3], vec![28, 29, 4]]);
        println!("{}", result);
        assert!((result - 30.78571).abs() < 1e-5,);
    }
}
