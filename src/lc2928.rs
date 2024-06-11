struct Solution;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Rest {
    children: i32,
    candies: i32,
}

impl Solution {
    // fn dfs(memo: &mut HashMap<Rest, i32>, rest: Rest, limit: i32) -> i32 {
    //     if rest.candies == 0 {
    //         1
    //     } else if rest.children != 0 {
    //         match memo.get(&rest) {
    //             Some(value) => *value,
    //             None => {
    //                 let mut result = 0;
    //                 let children = rest.children - 1;
    //                 for distributed in 0..=rest.candies.min(limit) {
    //                     let candies = rest.candies - distributed;
    //                     result += Self::dfs(memo, Rest { children, candies }, limit);
    //                 }
    //                 result
    //             }
    //         }
    //     } else {
    //         0
    //     }
    // }
    // fn dfs(memo: &mut HashMap<Rest, i32>, rest: Rest, limit: i32) -> i32 {
    //     if rest.candies == 0 {
    //         1
    //     } else if rest.children != 0 {
    //         use std::collections::hash_map::Entry;
    //         match memo.entry(rest) {
    //             Entry::Occupied(entry) => *entry.get(),
    //             Entry::Vacant(entry) => {
    //                 let mut result = 0;
    //                 let children = rest.children - 1;
    //                 for current in 0..=rest.candies.min(limit) {
    //                     let candies = rest.candies - current;
    //                     result += Self::dfs(memo, Rest { children, candies }, limit);
    //                 }
    //                 *entry.insert(result)
    //             }
    //         }
    //     } else {
    //         0
    //     }
    // }
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let no_candies = n as usize;
        let no_children = 3;
        let limit = limit as usize;
        let mut dp = vec![vec![0; no_candies + 1]; no_children + 1];
        dp[0][0] = 1;
        // for line in dp.iter_mut() {
        //     line[0] = 1;
        // }
        // for (children, line) in dp.iter_mut().enumerate() {
        //     for (candies, cell) in line.iter_mut().enumerate() {
        //         let need_distributed = candies.min(limit);
        //         // let result= dp[children - 1].iter().skip(candies - need_distributed).take(need_distributed + 1).sum();
        //         let start = candies - need_distributed;
        //         *cell = dp[children - 1][start..=candies].iter().sum();
        //     }
        // }
        for children in 1..=no_children {
            // for (candies, cell) in dp[children].iter_mut().enumerate() {
            //     let need_distributed = candies.min(limit);
            //     // let result= dp[children - 1].iter().skip(candies - need_distributed).take(need_distributed + 1).sum();
            //     let start = candies - need_distributed;
            //     *cell = dp[children - 1][start..=candies].iter().sum();
            // }
            for candies in 0..=no_candies {
                let need_distributed = candies.min(limit);
                // let result= dp[children - 1].iter().skip(candies - need_distributed).take(need_distributed + 1).sum();
                let start = candies - need_distributed;
                let result = dp[children - 1][start..=candies].iter().sum();
                // let mut result = 0;
                // for distributed in 0..=candies.min(limit) {
                //     result += dp[children - 1][candies - distributed];
                // }
                dp[children][candies] = result;
            }
        }
        dp[no_children][no_candies]

        // for first in 0..=n.min(limit) {
        //     let after_first = n - first;
        //     for second in 0..=after_first.min(limit) {
        //         let after_second = after_first - second;
        //         for third in 0..=after_second.min(limit) {
        //             let after_third = after_second - third;
        //             if after_third == 0 {
        //                 result += 1;
        //             }
        //         }
        //     }
        // }
        // result

        // let mut memo = HashMap::new();
        // let rest = Rest {
        //     children: 3,
        //     candies: n,
        // };
        // Self::dfs(&mut memo, rest, limit)
    }
}

// impl Solution {

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::distribute_candies(5, 2), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::distribute_candies(3, 3), 10);
    }
}
