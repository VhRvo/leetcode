struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut total = (m + n - 1) as usize;
        let mut left = m - 1;
        let mut right = n - 1;
        while left >= 0 && right >= 0 {
            if nums1[left as usize] > nums2[right as usize] {
                nums1[total] = nums1[left as usize];
                total -= 1;
                left -= 1;
            } else {
                nums1[total] = nums2[right as usize];
                total -= 1;
                right -= 1;
            }
        }
        while right >= 0 {
            nums1[total] = nums2[right as usize];
            total -= 1;
            right -= 1;
        }
    }
}
