struct Solution;

impl Solution {
    fn intersection_square_length(
        bottom_left1: (i32, i32),
        top_right1: (i32, i32),
        bottom_left2: (i32, i32),
        top_right2: (i32, i32),
    ) -> i32 {
        let height_overlap = top_right1.1.min(top_right2.1) - bottom_left1.1.max(bottom_left2.1);
        let width_overlap = top_right1.0.min(top_right2.0) - bottom_left1.0.max(bottom_left2.0);
        height_overlap.min(width_overlap)
    }
    fn array_to_tuple(array: &[i32]) -> (i32, i32) {
        (array[0], array[1])
    }
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let len = bottom_left.len();
        // You need to find the maximum area of a square that can fit inside the intersecting region of at least two rectangles.
        let mut result = 0;
        for ii in 0..len {
            for jj in ii + 1..len {
                let bottom_left1 = Self::array_to_tuple(&bottom_left[ii]);
                let bottom_left2 = Self::array_to_tuple(&bottom_left[jj]);
                let top_right1 = Self::array_to_tuple(&top_right[ii]);
                let top_right2 = Self::array_to_tuple(&top_right[jj]);
                let intersection_side_length = Self::intersection_square_length(
                    bottom_left1,
                    top_right1,
                    bottom_left2,
                    top_right2,
                );
                result = result.max(intersection_side_length);
            }
        }
        let result = result as i64;
        result * result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![2, 2], vec![3, 1]],
                vec![vec![3, 3], vec![4, 4], vec![6, 6]]
            ),
            1
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![1, 3], vec![1, 5]],
                vec![vec![5, 5], vec![5, 7], vec![5, 9]]
            ),
            4
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![2, 2], vec![1, 2]],
                vec![vec![3, 3], vec![4, 4], vec![3, 4]]
            ),
            1
        )
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::largest_square_area(
                vec![vec![1, 1], vec![3, 3], vec![3, 1]],
                vec![vec![2, 2], vec![4, 4], vec![4, 2]]
            ),
            0
        )
    }
}
