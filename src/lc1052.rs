struct Solution;
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        #[inline]
        fn calc((customer, grumpy): (&i32, &i32)) -> i32 {
            customer * grumpy
        }
        #[inline]
        fn slice_first_calc(left: &[i32], right: &[i32]) -> i32 {
            calc((left.first().unwrap(), right.first().unwrap()))
        }
        #[inline]
        fn slice_last_calc(left: &[i32], right: &[i32]) -> i32 {
            calc((left.last().unwrap(), right.last().unwrap()))
        }
        let result: i32 = customers
            .iter()
            .zip(grumpy.iter())
            .clone()
            .map(|(customer, grumpy)| customer * (1 - grumpy))
            .sum();

        let minutes = minutes as usize;
        let mut iter = customers.windows(minutes).zip(grumpy.windows(minutes));
        if let Some((customers_window, grumpy_window)) = iter.next() {
            let mut max_window_value: i32 =
                customers_window.iter().zip(grumpy_window).map(calc).sum();
            let mut maintained =
                max_window_value - slice_first_calc(customers_window, grumpy_window);
            for (customers_window, grumpy_window) in iter {
                maintained += slice_last_calc(customers_window, grumpy_window);
                max_window_value = max_window_value.max(maintained);
                maintained -= slice_first_calc(customers_window, grumpy_window);
            }
            result + max_window_value
        } else {
            result
        }
    }
}
