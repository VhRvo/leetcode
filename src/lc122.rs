struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut held = false;
        let mut buy_price = 0;
        let mut sell_price = i32::MAX;

        for price in prices {
            if held {
                if price < buy_price {
                    buy_price = price;
                } else {
                    sell_price = price;
                    profit += price - buy_price;
                    held = false;
                }
            } else if sell_price < price {
                profit += price - sell_price;
                sell_price = price;
            } else {
                held = true;
                buy_price = price;
            }
        }

        profit
    }
}
