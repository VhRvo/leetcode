struct Solution;

impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        sentence
            .split_whitespace()
            .map(|str| {
                if let Some(rest) = str.strip_prefix('$') {
                    if !rest.contains('e') {
                        if let Ok(value) = rest.parse::<f64>() {
                            format!("${:.2}", value / 100_f64 * ((100 - discount) as f64))
                        } else {
                            str.to_string()
                        }
                    } else {
                        str.to_string()
                    }
                } else {
                    str.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::discount_prices("there are $1 $2 and 5$ candies in the shop".to_string(), 50),
            "there are $0.50 $1.00 and 5$ candies in the shop"
        );
        assert_eq!(
            Solution::discount_prices("1 2 $3 4 $5 $6 7 8$ $9 $10$".to_string(), 100),
            "1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$"
        );
        assert_eq!(Solution::discount_prices("$1e9".to_string(), 100), "$1e9");
    }
}
