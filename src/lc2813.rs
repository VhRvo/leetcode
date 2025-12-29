struct Solution;

impl Solution {
    pub fn find_maximum_elegance(mut items: Vec<Vec<i32>>, k: i32) -> i64 {
        use std::collections::HashSet;

        let k = k as usize;
        let mut total_profit = 0_i64;
        let mut met = HashSet::new();
        let mut candidates = Vec::new();

        items.sort_by(|items1, item2| item2[0].cmp(&items1[0]));
        let (first, rest) = items.split_at(k);

        for item in first {
            let profit = item[0] as i64;
            let category = item[1];
            total_profit += profit;
            if !met.insert(category) {
                candidates.push(profit);
            }
        }

        let mut result = total_profit + (met.len() * met.len()) as i64;
        for item in rest.iter() {
            if candidates.is_empty() {
                break;
            }
            let profit = item[0] as i64;
            let category = item[1];
            // Quantitative change leads to qualitative change.
            if !met.contains(&category) {
                total_profit += profit - candidates.pop().unwrap();
                met.insert(category);
                result = result.max(total_profit + (met.len() * met.len()) as i64);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_maximum_elegance(vec![vec![3, 2], vec![5, 1], vec![10, 1]], 2),
            17
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_maximum_elegance(
                vec![vec![3, 1], vec![3, 1], vec![2, 2], vec![5, 3]],
                3
            ),
            19
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_maximum_elegance(vec![vec![1, 1], vec![2, 1], vec![3, 1]], 3),
            7
        );
    }
    #[test]
    fn test4() {
        assert_eq!(
            Solution::find_maximum_elegance(
                vec![vec![7, 1], vec![4, 1], vec![3, 2], vec![10, 1]],
                3
            ),
            24
        );
    }
    #[test]
    fn test5() {
        assert_eq!(
            Solution::find_maximum_elegance(
                vec![vec![2, 5], vec![2, 2], vec![7, 5], vec![2, 4], vec![6, 5]],
                2
            ),
            14
        );
    }
    #[test]
    fn test6() {
        assert_eq!(
            Solution::find_maximum_elegance(
                vec![
                    vec![10, 1],
                    vec![10, 1],
                    vec![10, 1],
                    vec![10, 1],
                    vec![10, 1],
                    vec![10, 1],
                    vec![10, 1],
                    vec![10, 1],
                    vec![10, 1],
                    vec![10, 1],
                    vec![3, 2],
                    vec![3, 3],
                    vec![3, 4],
                    vec![3, 5],
                    vec![3, 6],
                    vec![3, 7],
                    vec![3, 8],
                    vec![3, 9],
                    vec![3, 10],
                    vec![3, 11]
                ],
                10
            ),
            137
        );
    }
}
