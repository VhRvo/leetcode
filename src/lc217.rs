use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        nums.iter().any(|num| !set.insert(num))
        // struct Empty;
        // let mut map = HashMap::new();
        // for num in nums {
        //     match map.entry(num) {
        //         Entry::Occupied(_) => {
        //             return true;
        //         }
        //         Entry::Vacant(entry) => {
        //             entry.insert(Empty);
        //         }
        //     }
        // }
        // false
    }
}
