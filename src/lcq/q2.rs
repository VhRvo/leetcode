struct Solution;

/*
abcdefghij 0123456789
a0cdefghij b123456789
  bdefghij c123456789
  bdefghij c123456789
  b1efghij cd23456789
    cfghij ed23456789
    c2ghij edf3456789
      dhij egf3456789
      d3ij egfh456789
        ej igfh456789
        e4 igfhj56789
           fgihj56789
           f5ihjg6789
             ghji6789
             g6jih789
               hij789
 */

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        let n = n as usize;
        for ii in 0..n {
            result.push(nums[ii]);
            result.push(nums[ii + n]);
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
            Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3),
            vec![2, 3, 5, 4, 1, 7]
        )
    }
}
