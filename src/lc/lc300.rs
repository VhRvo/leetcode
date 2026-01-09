struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        Self::dp_from_start_patience_sorting(nums)
    }
    fn dp_length_maps_to_minimal_from_start(nums: Vec<i32>) -> i32 {
        // let mut dp = vec![0; nums.len()];
        // let mut length_to_minimal = Vec::with_capacity(nums.len());
        // for ii in 0..nums.len() {
        //     let longest = match length_to_minimal.binary_search(&nums[ii]) {
        //         Ok(jj) => {
        //             length_to_minimal[jj] = nums[ii];
        //             jj as i32
        //         }
        //         Err(_) => {
        //             length_to_minimal.push(nums[ii]);
        //             length_to_minimal.len() as i32
        //         }
        //     };
        //     dp[ii] = longest + 1;
        // }
        // length_to_minimal.len() as i32
        todo!()
    }
    fn dp_2d_from_start(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut result = 0;
        for ii in 0..nums.len() {
            let longest = (0..ii)
                .filter_map(|jj| {
                    if nums[jj] < nums[ii] {
                        Some(dp[jj])
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(0);
            dp[ii] = longest + 1;
            result = result.max(dp[ii]);
        }
        result
    }
    fn dp_from_end_patience_sorting(nums: Vec<i32>) -> i32 {
        let mut dp = Vec::with_capacity(nums.len());
        for ii in (0..nums.len()).rev() {
            let partition = dp.partition_point(|&element: &i32| {
                nums[ii] < element
            });
            if partition == dp.len() {
                dp.push(nums[ii]);
            } else {
                dp[partition] = dp[partition].max(nums[ii])
            }
        }
        dp.len() as i32
    }
    fn dp_from_start_patience_sorting(nums: Vec<i32>) -> i32 {
        let mut dp = Vec::with_capacity(nums.len());
        for ii in 0..nums.len() {
            let partition = dp.partition_point(|&element: &i32| {
                element < nums[ii]
            });
            if partition == dp.len() {
                dp.push(nums[ii]);
            } else {
                dp[partition] = dp[partition].min(nums[ii])
            }
        }
        dp.len() as i32
    }
    fn dp_value_maps_longest_from_end(_: Vec<i32>) -> i32 {
        // think: 4, 6, 7, 8, 5, 6
        // 6 |-> 1
        // 5 |-> 2
        // 8 |-> 1
        // 7 |-> 2
        // 6 |-> 3
        // map.range(Excluded(4), Unbounded).next() is Some((5, 2)), not Some((6, 3))
        // 4 |-> 4
        // have to iterate map.range(Excluded(nums[ii]), Unbounded), which is O(n)
        // dp[ii] = 1 + max(dp[jj] for jj in ii+1..n if nums[ii] < nums[jj])
        // the use of `map.range` is equivalent to `if nums[jj] < nums[jj]`
        // we cannot avoid the inner loop
        // the update `map[nums[ii]] = dp[ii]` is partial, cannot affect other keys
        // maybe we need fenwick tree or segment tree to speed up the range max query
        panic!()
    }
    fn dp_2d_from_end(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1];
        let mut result = 0;
        for ii in (0..nums.len()).rev() {
            let mut last_index = nums.len();
            for jj in (ii + 1..nums.len()).rev() {
                if nums[ii] < nums[jj] {
                    if dp[jj] >= dp[last_index] {
                        last_index = jj;
                    }
                }
            }
            dp[ii] = 1 + dp[last_index];
            result = result.max(dp[ii]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4)
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1)
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6)
    }
}
