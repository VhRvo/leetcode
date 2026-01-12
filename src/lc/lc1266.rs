struct Solution;

impl Solution {
    fn calulate_minimal_distance((x, y): (i32, i32), (next_x, next_y): (i32, i32)) -> i32 {
        let dx = (next_x - x).abs();
        let dy = (next_y - y).abs();
        return dx.max(dy);
    }
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let first = &points[0];
        let mut current = (first[0], first[1]);
        let mut result = 0;
        for point in points.into_iter().skip(1) {
            let next = (point[0], point[1]);
            result += Self::calulate_minimal_distance(current, next);
            current = next;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
            7
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
            5
        )
    }
}
