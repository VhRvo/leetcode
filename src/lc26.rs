use std::cell::Cell;

struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        let nums = Cell::from_mut(nums.as_mut_slice()).as_slice_of_cells();
        match nums {
            [first, rest @ ..] => {
                let mut previous = first.get();
                let mut position = 0_usize;
                for value in rest {
                    let current = value.get();
                    if previous != current {
                        unsafe {
                            rest.get_unchecked(position).set(current);
                        }
                        previous = current;
                        position += 1
                    }
                }
                (position + 1) as i32
            }
            _ => 0,
        }
        // let mut previous = *nums.first().unwrap();
        // let mut position = 1_usize;
        // for ii in (1..nums.len()) {
        //     if previous != nums[ii] {
        //         nums[position] = nums[ii];
        //         previous = nums[ii];
        //         position += 1;
        //     }
        // }
        // for value in nums.iter().skip(1) {
        //
        // }
        // position as i32
    }
}
