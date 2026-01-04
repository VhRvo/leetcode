struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        use std::cell::Cell;

        let length = s.len();
        let slice = Cell::from_mut(s.as_mut_slice()).as_slice_of_cells();
        // for ii in 0..length / 2 {
        //     slice[ii].swap(&slice[length - ii - 1]);
        // }
        let half = length / 2;
        for (front, rest) in slice.iter().take(half).zip(slice.iter().rev()) {
            front.swap(rest);
        }
    }
    // pub fn reverse_string(s: &mut Vec<char>) {
    //     let length = s.len();
    //     for ii in (0..length / 2) {
    //         s.swap(ii, length - ii - 1)
    //     }
    // }
}
