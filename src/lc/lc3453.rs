struct Solution;

impl Solution {
    fn calculate_area(line: i32, squares: &[Vec<i32>]) -> (i64, i64) {
        let line = line as i64;
        let mut below = 0;
        let mut above = 0;
        for sqaure in squares.iter() {
            let y = sqaure[1] as i64;
            let length = sqaure[2] as i64;
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
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        Self::partition_point_is_last_predicate(squares)
        // Self::partition_point_is_first_not_predicate(squares)
    }
    fn partition_point_is_last_predicate(squares: Vec<Vec<i32>>) -> f64 {
        // This question asks for the minimum y-coordinate of the separating line
        // This function returns the maximum y-coordinate of the separating line
        fn calculate_line_area(line: i32, squares: &[Vec<i32>]) -> i64 {
            let mut area = 0;
            for sqaure in squares.iter() {
                let y = sqaure[1];
                let length = sqaure[2];
                // y + length >= line + 1 => y + length > line
                if y <= line && y + length > line {
                    area += length as i64;
                }
            }
            area
        }
        let (lo, hi) = {
            let mut lowerest = i32::MAX;
            let mut upperest = i32::MIN;
            for square in squares.iter() {
                lowerest = lowerest.min(square[1]);
                upperest = upperest.max(square[1] + square[2]);
            }
            (lowerest, upperest)
        };
        // Predicate: returns true if the area below the line is less than or equal to the area above
        // We use this to find the last position where below <= above (i.e., the transition point)
        let predicate = |line: i32| {
            let (below, above) = Self::calculate_area(line, &squares);
            below <= above
        };
        let mi = {
            // Binary search to find the transition point
            // In the beginning: predicate(lo) is true, predicate(hi) is false
            let mut lo = lo;
            let mut hi = hi + 1;
            // Find the first position where predicate is false
            while lo < hi {
                let mi = lo.midpoint(hi);
                if predicate(mi) {
                    lo = mi + 1;
                } else {
                    hi = mi;
                }
            }
            lo - 1
        };
        let (below, above) = Self::calculate_area(mi, &squares);
        let area = calculate_line_area(mi, &squares);
        if below == above {
            mi as f64
        } else {
            mi as f64 + ((above - below) as f64 / (2 * area) as f64)
        }
    }
    fn partition_point_is_first_not_predicate(squares: Vec<Vec<i32>>) -> f64 {
        fn calculate_line_area(line: i32, squares: &[Vec<i32>]) -> i64 {
            let mut area = 0;
            for sqaure in squares.iter() {
                let y = sqaure[1];
                let length = sqaure[2];
                // y <= line - 1 => y < line
                if y < line && y + length >= line {
                    area += length as i64;
                }
            }
            area
        }
        let (lo, hi) = {
            let mut lowerest = i32::MAX;
            let mut upperest = i32::MIN;
            for square in squares.iter() {
                lowerest = lowerest.min(square[1]);
                upperest = upperest.max(square[1] + square[2]);
            }
            (lowerest, upperest)
        };
        // Predicate: returns true if the area below the line is strictly less than the area above
        // We use this to find the first position where below >= above
        let predicate = |line: i32| {
            let (below, above) = Self::calculate_area(line, &squares);
            below < above
        };
        let mi = {
            // Binary search to find the first position where predicate is false
            // In the beginning: predicate(lo) is true, predicate(hi) is false
            let mut lo = lo;
            let mut hi = hi + 1;
            // Find the first position where predicate is false
            while lo < hi {
                let mi = lo.midpoint(hi);
                if predicate(mi) {
                    lo = mi + 1;
                } else {
                    hi = mi;
                }
            }
            lo
        };
        let (below, above) = Self::calculate_area(mi, &squares);
        let area = calculate_line_area(mi, &squares);
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

    #[test]
    fn test4() {
        let result = Solution::separate_squares(vec![
            vec![522261215, 954313664, 225462],
            vec![628661372, 718610752, 10667],
            vec![619734768, 941310679, 44788],
            vec![352367502, 656774918, 289036],
            vec![860247066, 905800565, 100123],
            vec![817623994, 962847576, 71460],
            vec![691552058, 782740602, 36271],
            vec![911356, 152015365, 513881],
            vec![462847044, 859151855, 233567],
            vec![672324240, 954509294, 685569],
        ]);
        println!("{}", result);
        assert!((result - 954521423.80202).abs() < 1e-5,);
    }
}
