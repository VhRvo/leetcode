struct Solution;

// native
/*
use std::i32;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut hold = vec![0; prices.len() + 1];
        let mut not_hold = vec![0; prices.len() + 1];
        hold[prices.len()] = i32::MIN / 2;
        not_hold[prices.len()] = 0;
        for i in (0..prices.len()).rev() {
            hold[i] = hold[i + 1].max(not_hold[i + 1] + prices[i]);
            not_hold[i] = not_hold[i + 1].max(hold[i + 1] - prices[i]);
        }
        not_hold[0]
    }
}
*/

// rolling array
use std::i32;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut hold = i32::MIN / 2;
        let mut not_hold = 0;
        for i in (0..prices.len()).rev() {
            hold = hold.max(not_hold + prices[i]);
            not_hold = not_hold.max(hold - prices[i]);
        }
        not_hold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7)
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4)
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0)
    }
}

// stupid
// impl Solution {
//     pub fn max_profit(prices: Vec<i32>) -> i32 {
//         let mut profit = 0;
//         let mut held = false;
//         let mut buy_price = 0;
//         let mut sell_price = i32::MAX;

//         for price in prices {
//             if held {
//                 if price < buy_price {
//                     buy_price = price;
//                 } else {
//                     sell_price = price;
//                     profit += price - buy_price;
//                     held = false;
//                 }
//             } else if sell_price < price {
//                 profit += price - sell_price;
//                 sell_price = price;
//             } else {
//                 held = true;
//                 buy_price = price;
//             }
//         }

//         profit
//     }
// }
