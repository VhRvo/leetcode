struct Solution;

impl Solution {
    pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const MODULE: i64 = 1e9 as i64 + 7;
        for query in queries {
            let (mut idx, end, step, value) = (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                query[3] as i64,
            );
            while idx <= end {
                nums[idx] = ((nums[idx] as i64 * value) % MODULE) as i32;
                idx += step;
            }
        }
        let mut result = 0;
        for num in nums {
            result ^= num;
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
            4,
            Solution::xor_after_queries(vec![1, 1, 1], vec![vec![0, 2, 1, 4]])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            31,
            Solution::xor_after_queries(
                vec![2, 3, 1, 5, 4],
                vec![vec![1, 4, 2, 3], vec![0, 2, 1, 2]]
            )
        );
    }
}
