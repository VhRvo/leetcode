struct Solution;

impl Solution {
    #[inline]
    fn sum(start: i32, end: i32) -> i32 {
        (start + end) * (end - start + 1) / 2
    }

    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut result = vec![0; num_people as usize];
        // let
        let mut rest_candies = candies;
        let mut base = 0;
        while rest_candies > 0 {
            for (index, item) in (1..=num_people).zip(result.iter_mut()) {
                if rest_candies <= 0 {
                    break;
                }
                let current_distributed = (index + base).min(rest_candies);
                *item += current_distributed;
                rest_candies -= current_distributed;
            }
            base += num_people;
        }
        result
        // let repeat_times = {
        //     let mut lo = 0;
        //     let mut hi = candies + 1;
        //     while hi - lo >= 1 {
        //         let mi = lo + (hi - lo) / 2;
        //         let distributed = Self::sum(1, mi);
        //         if distributed <= candies {
        //             lo = mi + 1;
        //         } else {
        //             hi = mi;
        //         }
        //     }
        //     lo
        // };
        // let distributed = Self::sum(1, repeat_times);
        // let rest_canides = candies - distributed;
        //
        // ; todo!()
    }
    // pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
    //     let once_distributed = Self::sum(1, num_people);
    //     let turns = {
    //         let mut lo = 0;
    //         let mut hi = candies;
    //         // while lo < hi  {
    //         while hi - lo >= 1 {
    //             let mi = lo + (hi - lo) / 2;
    //             let distributed = Self::sum(1, mi) * once_distributed;
    //             if distributed <= candies {
    //                 lo = mi + 1;
    //             } else {
    //                 hi = mi;
    //             }
    //         }
    //         lo
    //     };
    //     let mut result = Vec::new();
    //     let distributed = Self::sum(1, turns);
    //     for ii in 1..=num_people {
    //         result.push(ii * distributed);
    //     }
    //     let mut rest_candies = candies - distributed;
    //     let last_turn = turns + 1;
    //     for (item, index) in result.iter_mut().zip(1..=num_people) {
    //         if rest_candies == 0 {
    //             break;
    //         }
    //         let current_distributed = (index * last_turn).min(candies);
    //         *item += current_distributed;
    //         rest_candies -= current_distributed;
    //     }
    //     result
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::distribute_candies(7, 4), vec![1, 2, 3, 1]);
        assert_eq!(Solution::distribute_candies(10, 3), vec![5, 2, 3]);
    }
}
