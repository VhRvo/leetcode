struct Solution;
impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut result = 0;
        for ii in 0..arr.len() {
            for jj in ii + 1..arr.len() {
                for kk in jj + 1..arr.len() {
                    if (arr[ii] - arr[jj]).abs() <= a
                        && (arr[jj] - arr[kk]).abs() <= b
                        && (arr[ii] - arr[kk]).abs() <= c
                    {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}
