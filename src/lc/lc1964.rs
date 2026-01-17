struct Solution;

impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(obstacles.len());
        let mut monotone_stack = Vec::new();
        for ii in 0..obstacles.len() {
            let height = obstacles[ii];
            let point = monotone_stack.partition_point(|&probe: &i32| probe <= height);
            if point == monotone_stack.len() {
                monotone_stack.push(height);
            } else {
                monotone_stack[point] = monotone_stack[point].min(height);
            }
            result.push(point as i32 + 1);
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
            Solution::longest_obstacle_course_at_each_position(vec![1, 2, 3, 2]),
            vec![1, 2, 3, 3]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(vec![2, 2, 1]),
            vec![1, 2, 1]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::longest_obstacle_course_at_each_position(vec![3, 1, 5, 6, 4, 2]),
            vec![1, 1, 2, 3, 2, 2]
        );
    }
}
