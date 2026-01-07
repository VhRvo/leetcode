struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp_longest = vec![0; nums.len() + 1];
        let mut dp_count = vec![1; nums.len() + 1];
        let mut longest_length = 0;


        for ii in (0..nums.len()).rev() {
            let mut last_index = nums.len();
            let mut count = dp_count[last_index];
            for jj in (ii + 1..nums.len()).rev() {
                if nums[ii] < nums[jj] {
                    if dp_longest[jj] > dp_longest[last_index] {
                        count = dp_count[jj];
                        last_index = jj;
                    } else if dp_longest[jj] == dp_longest[last_index] {
                        count += dp_count[jj];
                    }
                }
            }
            dp_longest[ii] = 1 + dp_longest[last_index];
            dp_count[ii] = count;
            longest_length = longest_length.max(dp_longest[ii]);
        }

        let mut result_count = 0;
        for ii in 0..nums.len() {
            if dp_longest[ii] == longest_length {
                result_count += dp_count[ii];
            }
        }
        result_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]), 5)
    }
}
