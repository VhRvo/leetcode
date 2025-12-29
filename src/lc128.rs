struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // use std::collections::HashMap;
        use std::collections::HashSet;

        let mut unique = HashSet::new();
        for num in nums.iter().copied() {
            unique.insert(num);
        }

        let mut result = 0;
        for num in nums.iter().copied() {
            if unique.get(&(num - 1)).is_none() {
                for (len, succ) in (num..).enumerate().skip(1) {
                    if unique.get(&succ).is_none() {
                        result = result.max(len as i32);
                        break;
                    }
                }
            }
        }
        result

        // let mut map = HashMap::new();
        // fn solve(
        //     base: i32,
        //     len: i32,
        //     map: &mut HashMap<i32, i32>,
        //     unique: &mut HashSet<i32>,
        // ) -> i32 {
        //     let num = base + len;
        //     match unique.get(&num) {
        //         None => 0,
        //         Some(_) => match map.get(&num) {
        //             None => {
        //                 let value = solve(base, len + 1, map, unique);
        //                 map.insert(num, value + 1);
        //                 value + 1
        //             }
        //             Some(value) => *value,
        //         },
        //     }
        // }
        // let mut result = 0;
        // for num in nums.iter().copied() {
        //     result = result.max(solve(num, 0, &mut map, &mut unique))
        // }
        // result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
